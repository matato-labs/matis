//! Measure source and dependency graphs, build reports, and evaluate project rules.

pub mod adapters;
mod architecture;
pub mod artifact;
pub mod callables;
pub mod check;
pub mod complexity;
pub mod config;
pub mod cycles;
pub mod dependency_metrics;
mod exception_policy;
pub(crate) mod forbidden;
pub mod receiver_resolution;
pub mod receiver_syntax;
pub mod report;
pub mod report_markdown;
pub mod resolution_contract;
pub mod resolution_syntax;
pub mod resolver;
