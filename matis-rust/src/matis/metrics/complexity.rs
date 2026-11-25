//! Measure decision, loop, and exception complexity using language-specific syntax policies.

use super::{
    adapters::base::{DEFAULT_METRICS, MetricPolicy},
    exception_policy as exceptions,
};
use crate::{domain::models::Complexity, syntax::named_children};
use tree_sitter::Node;

/// Calculate entry, decision, deep-decision and nested-loop costs without recursion.
/// Use `policy` to classify nodes in `root` and `source` for source-dependent predicates.
/// Exclude nested callable/type bodies and keep else-if alternatives at the same depth.
/// Include each nested exception handler once in weighted complexity.
pub fn calculate_complexity(root: Node<'_>, policy: &MetricPolicy, source: &[u8]) -> Complexity {
    let root = policy.body(root);
    let (mut base, mut nested, mut loops, mut maximum) = (1, 0, 0, 0);
    let applicable = exceptions::applicable(&policy.language);
    let mut exception_count = 0;
    let mut stack = vec![(root, 0usize, 0usize, applicable.then_some(false))];
    while let Some((node, mut depth, loop_depth, in_handler)) = stack.pop() {
        if node != root && policy.scope(node, source) {
            continue;
        }
        depth = depth.saturating_sub(usize::from(policy.alternative(node)));
        let decision = policy.decision(node, source);
        depth += usize::from(decision);
        base += usize::from(decision) + usize::from(policy.logical(node));
        nested += usize::from(decision && depth > 2);
        loops += usize::from(policy.is_loop(node) && loop_depth > 0);
        maximum = maximum.max(depth);
        let child_loop_depth = loop_depth + usize::from(policy.is_loop(node));
        let in_handler = in_handler.filter(|_| node == root || !exceptions::extra_scope(&policy.language, node));
        let in_handler = in_handler.map(|inside| {
            let handles = exceptions::handles(&policy.language, node, source);
            exception_count += usize::from(inside && handles);
            inside || handles
        });
        stack.extend(named_children(node).into_iter().rev().map(|child| (child, depth, child_loop_depth, in_handler)));
    }
    Complexity {
        base_cyclomatic_complexity: base,
        nested_cyclomatic_complexity: nested,
        loop_nested_complexity: loops,
        nested_try_catches: applicable.then_some(exception_count),
        cyclomatic_complexity: base + nested + loops + exception_count,
        max_decision_depth: maximum,
    }
}

/// Measure `root` against `source` using the merged default grammar policies.
/// Without a selected language, the exception nesting metric is unavailable.
pub fn default_complexity(root: Node<'_>, source: &[u8]) -> Complexity {
    calculate_complexity(root, &DEFAULT_METRICS, source)
}
