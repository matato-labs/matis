//! Read and replace saved analysis diagnostics through the diagnostic store.

use crate::domain::storage::DiagnosticPort;
use anyhow::Result;
use serde_json::Value;
/// Owns storage for analysis diagnostics from the latest saved graph.
pub struct DiagnosticService {
    repository: Box<dyn DiagnosticPort>,
}
impl DiagnosticService {
    /// Bind the diagnostic store to the workflow's transaction scope.
    pub fn new(repository: impl DiagnosticPort + 'static) -> Self {
        Self { repository: Box::new(repository) }
    }
    /// Read the saved diagnostics for a repository.
    pub(crate) fn all(&self, id: &str) -> Result<Vec<Value>> {
        self.repository.all(id)
    }
    /// Read up to `limit` diagnostics for the selected source paths.
    pub(crate) fn paths(&self, id: &str, paths: &[String], limit: usize) -> Result<Vec<Value>> {
        self.repository.paths(id, paths, limit)
    }
    /// Count saved diagnostics with error severity.
    pub(crate) fn errors(&self, id: &str) -> Result<i64> {
        self.repository.errors(id)
    }
    /// Replace the repository's diagnostic rows with those in the new artifact.
    pub(crate) fn replace(&self, id: &str, artifact: &Value) -> Result<()> {
        self.repository.replace(id, artifact)
    }
}
