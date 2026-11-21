//! Track the metadata and revision ID of each repository's latest saved graph.

use crate::domain::repositories::GraphDto;
use crate::domain::storage::GraphRevisionPort;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
/// Owns storage for graph revision metadata, not graph contents.
pub struct GraphRevisionService {
    repository: Box<dyn GraphRevisionPort>,
}
impl GraphRevisionService {
    /// Bind the revision store to the workflow's transaction scope.
    pub fn new(repository: impl GraphRevisionPort + 'static) -> Self {
        Self { repository: Box::new(repository) }
    }
    /// Look up the current saved graph metadata by repository ID.
    pub(crate) fn get(&self, id: &str) -> Result<Option<GraphDto>> {
        self.repository.get(id)
    }
    /// Return latest import timestamps keyed by repository ID.
    pub(crate) fn imports(&self) -> Result<HashMap<String, i64>> {
        self.repository.imports()
    }
    /// Save new revision metadata in the caller's graph replacement transaction.
    pub(crate) fn save(&self, id: &str, artifact: &Value) -> Result<GraphDto> {
        self.repository.save(id, artifact)
    }
}
