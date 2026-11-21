//! Persist and read candidates discovered during source analysis.

use crate::domain::storage::MutationCandidatePort;
use anyhow::Result;
use serde_json::Value;
/// Owns candidate storage without running mutations or reading live source.
pub struct MutationCandidateService {
    repository: Box<dyn MutationCandidatePort>,
}
impl MutationCandidateService {
    /// Bind the candidate store to the workflow's transaction scope.
    pub fn new(repository: impl MutationCandidatePort + 'static) -> Self {
        Self { repository: Box::new(repository) }
    }
    /// Read saved candidates in artifact order.
    pub(crate) fn all(&self, id: &str) -> Result<Vec<Value>> {
        self.repository.all(id)
    }
    /// Insert candidate rows for the new graph after its file rows exist.
    pub(crate) fn insert(&self, id: &str, artifact: &Value) -> Result<()> {
        self.repository.insert(id, artifact)
    }
}
