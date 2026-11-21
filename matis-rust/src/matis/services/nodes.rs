//! Combine hierarchy records with saved file, symbol, and decorator details.

use crate::domain::{
    navigation::{ChildrenRequest, error},
    storage::{DecoratorPort, FilePort, NodePort, SymbolPort},
};
use anyhow::Result;
use serde_json::{Value, json};
/// Stores that must share the node workflow's transaction scope.
pub struct NodeRepositories {
    /// Base graph hierarchy and node identities.
    pub nodes: Box<dyn NodePort>,
    /// File metadata keyed by node identity.
    pub files: Box<dyn FilePort>,
    /// Source symbol details keyed by node identity.
    pub symbols: Box<dyn SymbolPort>,
    /// Ordered decorators associated with symbols.
    pub decorators: Box<dyn DecoratorPort>,
}
/// Owns the stores for hierarchy and node details.
pub struct NodeService {
    nodes: Box<dyn NodePort>,
    files: Box<dyn FilePort>,
    symbols: Box<dyn SymbolPort>,
    decorators: Box<dyn DecoratorPort>,
}
impl NodeService {
    /// Bind the stores used to read or replace graph nodes and their details.
    pub fn new(repositories: NodeRepositories) -> Self {
        let NodeRepositories { nodes, files, symbols, decorators } = repositories;
        Self { nodes, files, symbols, decorators }
    }
    fn symbols(&self, id: &str, mut nodes: Vec<Value>) -> Result<Vec<Value>> {
        let keys: Vec<_> = nodes.iter().map(|n| n["stableKey"].as_str().unwrap().to_owned()).collect();
        let mut symbols = self.symbols.get_many(id, &keys)?;
        for node in &mut nodes {
            if let Some(symbol) = symbols.remove(node["stableKey"].as_str().unwrap()) {
                node.as_object_mut().unwrap().extend(symbol.as_object().unwrap().clone());
            }
        }
        Ok(nodes)
    }
    /// Read all base nodes with their symbol details merged in.
    pub(crate) fn all(&self, id: &str) -> Result<Vec<Value>> {
        self.symbols(id, self.nodes.all(id)?)
    }
    /// Count nodes by graph level.
    pub(crate) fn levels(&self, id: &str) -> Result<Vec<(String, i64)>> {
        self.nodes.levels(id)
    }
    /// Count stored symbol records, not every graph node.
    pub(crate) fn symbol_count(&self, id: &str) -> Result<i64> {
        self.symbols.count(id)
    }
    /// Read a bounded set of roots with symbol details.
    pub(crate) fn roots(&self, id: &str, limit: usize) -> Result<Vec<Value>> {
        self.symbols(id, self.nodes.roots(id, limit)?)
    }
    /// Search stored nodes and attach symbol details to the bounded result.
    pub(crate) fn find(&self, id: &str, query: &str, limit: usize) -> Result<Vec<Value>> {
        self.symbols(id, self.nodes.find(id, query, limit)?)
    }
    /// Read a page of direct children with symbol details.
    pub(crate) fn children(&self, id: &str, request: &ChildrenRequest<'_>) -> Result<Vec<Value>> {
        self.symbols(id, self.nodes.children(id, request)?)
    }
    /// Read one node by stable key with file metadata and up to 100 decorators. Fail if the key is missing.
    pub(crate) fn node(&self, id: &str, key: &str) -> Result<Value> {
        let node = self
            .nodes
            .get(id, key)?
            .ok_or_else(|| error(format!("Node not found: {key}. Use find to obtain a stableKey.")))?;
        let mut node = self.symbols(id, vec![node])?.remove(0);
        node["external"] = json!(node["level"] == "external");
        node["file"] = self.files.get_many(id, &[key.to_owned()])?.remove(key).unwrap_or(Value::Null);
        let mut decorators = self.decorators.get(id, key, 101)?;
        node["decoratorsTruncated"] = json!(decorators.len() > 100);
        decorators.truncate(100);
        node["decorators"] = json!(decorators);
        Ok(node)
    }
    /// Return up to two matching file keys and saved hashes so the caller can detect ambiguity.
    pub(crate) fn file(&self, id: &str, path: &str) -> Result<Vec<Value>> {
        let nodes = self.nodes.files_at(id, path, 2)?;
        let keys: Vec<_> = nodes.iter().map(|n| n["stableKey"].as_str().unwrap().to_owned()).collect();
        let files = self.files.get_many(id, &keys)?;
        Ok(nodes
            .into_iter()
            .filter_map(|n| {
                let key = n["stableKey"].as_str().unwrap();
                files.get(key).map(|f| json!({"stableKey":key,"sha256":f["sha256"]}))
            })
            .collect())
    }
    /// Replace the node hierarchy, then insert file, symbol, and decorator details in the caller's transaction.
    pub(crate) fn replace(&self, id: &str, artifact: &Value) -> Result<()> {
        self.nodes.clear(id)?;
        self.nodes.insert(id, artifact)?;
        self.files.insert(id, artifact)?;
        self.symbols.insert(id, artifact)?;
        self.decorators.insert(id, artifact)
    }
}
