//! Store measured node properties separately from the graph hierarchy.

use crate::domain::storage::NodeMetricsPort;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
/// Owns storage for metrics associated with saved graph nodes.
pub struct NodeMetricsService {
    repository: Box<dyn NodeMetricsPort>,
}
impl NodeMetricsService {
    /// Bind the node metrics store to the workflow's transaction scope.
    pub fn new(repository: impl NodeMetricsPort + 'static) -> Self {
        Self { repository: Box::new(repository) }
    }
    /// Read saved metrics keyed by node stable key.
    pub(crate) fn all(&self, id: &str) -> Result<HashMap<String, Value>> {
        self.repository.all(id)
    }
    /// Read the stored metrics for one node stable key.
    pub(crate) fn get(&self, id: &str, key: &str) -> Result<Value> {
        self.repository.get(id, key)
    }
    /// Insert metrics from the artifact after its nodes exist.
    pub(crate) fn insert(&self, id: &str, artifact: &Value) -> Result<()> {
        self.repository.insert(id, artifact)
    }
}
