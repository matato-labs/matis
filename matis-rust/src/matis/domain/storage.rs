//! Storage contracts for repository and graph services. The caller supplies the active transaction or read snapshot.

use super::{
    navigation::{AdjacencyFilter, ChildrenRequest, EdgeRequest},
    repositories::{GraphDto, RepositoryDto},
};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

/// Owns the commit. Dropping an unfinished scope rolls back its writes.
pub trait Transaction: Send {
    /// Commit all writes in this scope and release it.
    fn commit(self: Box<Self>) -> Result<()>;
}
/// Opens the transaction shared by the workflow's storage capabilities.
pub trait UnitOfWork: Send {
    /// Begin a scope. Implementations may reject a nested scope.
    fn begin(&self) -> Result<Box<dyn Transaction>>;
}
/// Repository registrations, independent of saved graph contents.
pub trait RepositoryPort: Send {
    /// Insert a validated name and location and return the assigned ID.
    fn register(&self, name: &str, location: &str) -> Result<RepositoryDto>;
    /// List registrations without graph import timestamps.
    fn list(&self) -> Result<Vec<RepositoryDto>>;
    /// Return whether the exact repository ID exists.
    fn exists(&self, id: &str) -> Result<bool>;
    /// Resolve a project name to a repository ID.
    fn resolve(&self, name: &str) -> Result<Option<String>>;
    /// Find matches by ID or name, with enough results to detect ambiguity.
    fn select(&self, name: &str) -> Result<Vec<RepositoryDto>>;
}
/// Metadata for the latest saved graph of each repository.
pub trait GraphRevisionPort: Send {
    /// Return the current normalized graph metadata, or `None` if no normalized graph is saved.
    fn get(&self, id: &str) -> Result<Option<GraphDto>>;
    /// Return graph import times keyed by repository ID.
    fn imports(&self) -> Result<HashMap<String, i64>>;
    /// Save revision metadata for a replacement graph in the active transaction.
    fn save(&self, id: &str, artifact: &Value) -> Result<GraphDto>;
}
/// Base graph nodes and hierarchy queries. File and symbol details use separate ports.
pub trait NodePort: Send {
    /// Read all base nodes in artifact order.
    fn all(&self, id: &str) -> Result<Vec<Value>>;
    /// Look up one node by stable key.
    fn get(&self, id: &str, key: &str) -> Result<Option<Value>>;
    /// Read a bounded set of root nodes.
    fn roots(&self, id: &str, limit: usize) -> Result<Vec<Value>>;
    /// Match an exact stable key or search paths, names, and qualified names. Limit the result.
    fn find(&self, id: &str, query: &str, limit: usize) -> Result<Vec<Value>>;
    /// Read direct children in artifact order using the requested offset and limit.
    fn children(&self, id: &str, request: &ChildrenRequest<'_>) -> Result<Vec<Value>>;
    /// Find a bounded set of file nodes with an exact path match.
    fn files_at(&self, id: &str, path: &str, limit: usize) -> Result<Vec<Value>>;
    /// Count nodes by graph level.
    fn levels(&self, id: &str) -> Result<Vec<(String, i64)>>;
    /// Delete base nodes before graph replacement.
    fn clear(&self, id: &str) -> Result<()>;
    /// Insert base nodes before their dependent rows.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// File details associated with saved graph nodes.
pub trait FilePort: Send {
    /// Read requested file metadata keyed by node stable key.
    fn get_many(&self, id: &str, keys: &[String]) -> Result<HashMap<String, Value>>;
    /// Insert file details for the new graph's file nodes.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Source symbol details associated with saved graph nodes.
pub trait SymbolPort: Send {
    /// Read requested symbol details keyed by node stable key.
    fn get_many(&self, id: &str, keys: &[String]) -> Result<HashMap<String, Value>>;
    /// Count saved symbol records.
    fn count(&self, id: &str) -> Result<i64>;
    /// Insert symbol details for the new graph's nodes.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Decorators associated with saved symbols.
pub trait DecoratorPort: Send {
    /// Read a bounded list of decorators for one stable key.
    fn get(&self, id: &str, key: &str, limit: usize) -> Result<Vec<String>>;
    /// Insert decorators after the corresponding symbol records exist.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Measured properties associated with saved graph nodes.
pub trait NodeMetricsPort: Send {
    /// Read metrics keyed by node stable key.
    fn all(&self, id: &str) -> Result<HashMap<String, Value>>;
    /// Read metrics for one node stable key.
    fn get(&self, id: &str, key: &str) -> Result<Value>;
    /// Insert metrics after the corresponding nodes exist.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Saved graph edges and bounded adjacency queries.
pub trait RelationshipPort: Send {
    /// Read all edges in artifact order.
    fn all(&self, id: &str) -> Result<Vec<Value>>;
    /// Count edges by relationship type.
    fn counts(&self, id: &str) -> Result<Vec<(String, i64)>>;
    /// Read a bounded adjacency result for one direction and relationship filter.
    fn adjacency(&self, id: &str, request: &EdgeRequest, filter: AdjacencyFilter<'_>) -> Result<Vec<Value>>;
    /// Delete old edges before replacing their endpoint nodes.
    fn clear(&self, id: &str) -> Result<()>;
    /// Insert edges once their endpoint nodes exist.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Diagnostics associated with the latest saved analysis.
pub trait DiagnosticPort: Send {
    /// Read every diagnostic in artifact order.
    fn all(&self, id: &str) -> Result<Vec<Value>>;
    /// Read diagnostics for the requested paths, bounded by `limit`.
    fn paths(&self, id: &str, paths: &[String], limit: usize) -> Result<Vec<Value>>;
    /// Count diagnostics with error severity.
    fn errors(&self, id: &str) -> Result<i64>;
    /// Replace diagnostics with the records from the new artifact.
    fn replace(&self, id: &str, artifact: &Value) -> Result<()>;
}
/// Mutation candidates discovered during analysis and linked to saved files.
pub trait MutationCandidatePort: Send {
    /// Read candidates in artifact order.
    fn all(&self, id: &str) -> Result<Vec<Value>>;
    /// Insert candidates after the corresponding file records exist.
    fn insert(&self, id: &str, artifact: &Value) -> Result<()>;
}
