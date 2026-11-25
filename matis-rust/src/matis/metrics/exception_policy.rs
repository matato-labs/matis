//! Classify lexical exception handlers, not cleanup-only or error propagation syntax.

use super::adapters::elixir;
use crate::syntax::{field, named_children, text};
use tree_sitter::Node;

const ELIXIR_IMPLICIT_HANDLERS: &[&str] = &["def", "defp", "defmacro", "defmacrop"];

fn handler_kind(language: &str, kind: &str) -> Option<&'static str> {
    match (language, kind) {
        ("python", "try_statement") => Some("except_clause"),
        ("javascript" | "jsx" | "typescript" | "tsx" | "java", "try_statement")
        | ("java", "try_with_resources_statement") => Some("catch_clause"),
        ("kotlin", "try_expression") => Some("catch_block"),
        _ => None,
    }
}

pub(crate) fn applicable(language: &str) -> bool {
    matches!(language, "python" | "javascript" | "jsx" | "typescript" | "tsx" | "java" | "kotlin" | "elixir")
}

pub(crate) fn handles(language: &str, node: Node<'_>, source: &[u8]) -> bool {
    if let Some(handler) = handler_kind(language, node.kind()) {
        return named_children(node).iter().any(|child| child.kind() == handler);
    }
    language == "elixir" && elixir_handles(node, source)
}

fn elixir_handles(node: Node<'_>, source: &[u8]) -> bool {
    let target = elixir::target(node, source);
    // The walker skips nested declarations before testing implicit handlers.
    if target != "try" && !ELIXIR_IMPLICIT_HANDLERS.contains(&target.as_str()) {
        return false;
    }
    named_children(node).iter().any(|child| match child.kind() {
        "do_block" => named_children(*child).iter().any(|part| matches!(part.kind(), "rescue_block" | "catch_block")),
        "arguments" => named_children(*child).into_iter().any(|option| elixir_options(option, source)),
        _ => false,
    })
}

/// Return the declaration head, whose defaults run outside the implicit body handler.
pub(crate) fn unprotected_head<'tree>(language: &str, root: Node<'tree>, source: &[u8]) -> Option<Node<'tree>> {
    if language != "elixir" || !ELIXIR_IMPLICIT_HANDLERS.contains(&elixir::target(root, source).as_str()) {
        return None;
    }
    elixir_expressions(elixir::arguments(Some(root))?).next()
}

fn elixir_expressions(node: Node<'_>) -> impl Iterator<Item = Node<'_>> {
    named_children(node).into_iter().filter(|child| child.kind() != "comment")
}

fn elixir_options(mut node: Node<'_>, source: &[u8]) -> bool {
    // A literal keyword list can be bracketed, with pairs written as atom tuples.
    while node.kind() == "block" {
        let mut expressions = elixir_expressions(node);
        let (Some(expression), None) = (expressions.next(), expressions.next()) else {
            return false;
        };
        node = expression;
    }
    match node.kind() {
        "keywords" => elixir_keywords(node, source),
        "list" => elixir_expressions(node).any(|entry| match entry.kind() {
            "keywords" => elixir_keywords(entry, source),
            "tuple" => elixir_tuple(entry, source),
            _ => false,
        }),
        _ => false,
    }
}

fn elixir_tuple(node: Node<'_>, source: &[u8]) -> bool {
    let mut expressions = elixir_expressions(node);
    let key = expressions.next();
    expressions.next().is_some() && expressions.next().is_none() && elixir_handler_key(key, source)
}

fn elixir_keywords(node: Node<'_>, source: &[u8]) -> bool {
    named_children(node).iter().any(|pair| elixir_handler_key(field(*pair, &["key"]), source))
}

fn elixir_handler_key(key: Option<Node<'_>>, source: &[u8]) -> bool {
    let raw = text(key, source);
    let raw = raw.trim_end();
    let name = raw.strip_prefix(':').or_else(|| raw.strip_suffix(':'));
    matches!(name, Some("rescue" | "catch" | "\"rescue\"" | "\"catch\"" | "'rescue'" | "'catch'"))
}

/// Boundaries beyond the shared complexity scopes; do not change other complexity components.
pub(crate) fn extra_scope(language: &str, node: Node<'_>) -> bool {
    matches!(
        (language, node.kind()),
        ("typescript" | "tsx", "abstract_class_declaration")
            | ("java" | "kotlin", "class_body" | "enum_body" | "enum_class_body")
            | (
                "java",
                "record_declaration"
                    | "annotation_type_declaration"
                    | "interface_declaration"
                    | "compact_constructor_declaration"
                    | "enum_declaration"
            )
            | ("kotlin", "companion_object" | "secondary_constructor" | "getter" | "setter")
    )
}
