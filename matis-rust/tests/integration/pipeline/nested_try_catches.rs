use crate::support::{cli::invoke, fixtures};
use matis::{
    indexer::analyze_repository,
    metrics::{check::build_check, report::build_artifact_report},
};
use serde_json::{Value, json};
use std::fs;

const RULE: &str = "maxNestedTryCatches";
const METRIC: &str = "nestedTryCatches";

#[test]
fn real_grammars_count_inner_constructs_not_handlers_or_separate_scopes() {
    let graph = analyze_repository(&fixtures().join("nested_try_catches")).unwrap();
    let report = build_artifact_report(&graph).unwrap();
    assert_eq!(report["diagnostics"], json!([]));
    let nodes = report["nodes"].as_array().unwrap();
    let findings = build_check(&report, &json!({})).unwrap();
    let violations: Vec<_> =
        findings["findings"].as_array().unwrap().iter().filter(|finding| finding["rule"] == RULE).collect();
    let expectations = [
        ("Exceptions.java", 4),
        ("exceptions.ex", 5),
        ("exceptions.kt", 4),
        ("exceptions.py", 5),
        ("script.js", 4),
        ("script.ts", 1),
        ("view.jsx", 1),
        ("view.tsx", 1),
    ];
    let cases = expectations
        .into_iter()
        .flat_map(|(path, count)| {
            [("nested", count), ("flat", 0), ("independent", 0)].map(|(name, want)| (path, name, want))
        })
        .chain([("exceptions.ex", "implicit", 2), ("exceptions.ex", "default_argument", 0)]);
    for (path, name, want) in cases {
        let node = nodes
            .iter()
            .find(|node| {
                node["path"] == path && node["subject"].as_str().unwrap().rsplit([':', '.']).next() == Some(name)
            })
            .unwrap_or_else(|| panic!("missing {path}::{name}"));
        let m = &node["metrics"];
        assert_eq!(m[METRIC], want, "{path}::{name}");
        let old_weight: u64 = ["cyclomaticComplexity", "nestedDecisionPenalty", "nestedLoops"]
            .into_iter()
            .map(|key| m[key].as_u64().unwrap())
            .sum();
        assert_eq!(m["weightedComplexity"], old_weight + want, "{path}::{name}");
        let finding = violations.iter().find(|finding| finding["subject"] == node["subject"]);
        assert_eq!(
            finding.and_then(|finding| finding["actual"].as_u64()),
            (want > 0).then_some(want),
            "{path}::{name}"
        );
    }
    assert_eq!(violations.len(), expectations.len() + 1, "one additional Elixir implicit handler");
    for path in ["errors.go", "errors.rs"] {
        let node = nodes.iter().find(|node| node["subject"] == format!("{path}::ordinary")).unwrap();
        assert!(node["metrics"].get(METRIC).is_none(), "{path}");
    }
}

#[test]
fn cli_enforces_default_for_old_configs_and_reports_the_count() {
    let root = tempfile::tempdir().unwrap();
    fs::copy(fixtures().join("nested_try_catches/view.jsx"), root.path().join("view.jsx")).unwrap();
    let folder = root.path().to_str().unwrap();
    let config = root.path().join("matis.json");
    for legacy in [None, Some(r#"{"maxNestedLoops":1}"#)] {
        if let Some(source) = legacy {
            fs::write(&config, source).unwrap();
        }
        let output = invoke(&["check", "--folder", folder, "--format", "json"]);
        assert_eq!(output.status.code(), Some(1));
        let findings: Value = serde_json::from_slice(&output.stdout).unwrap();
        let findings = findings["findings"].as_array().unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0]["rule"], RULE);
        assert_eq!(findings[0]["metric"], METRIC);
        assert_eq!(findings[0]["files"], json!([{"path": "view.jsx", "line": 1}]));
        assert_eq!(findings[0]["actual"], 1);
        assert_eq!(findings[0]["limit"], 0);
        assert!(findings[0]["helpUrl"].as_str().unwrap().ends_with("#maxnestedtrycatches"));
    }
    let markdown = invoke(&["check", "--folder", folder]);
    assert_eq!(markdown.status.code(), Some(1));
    let markdown = String::from_utf8(markdown.stdout).unwrap();
    assert!(markdown.contains(RULE) && markdown.contains("Nested try/catch"));
    assert!(markdown.contains("view.jsx:1"));
    for limit in ["1", "null"] {
        fs::write(&config, format!(r#"{{"{RULE}":{limit}}}"#)).unwrap();
        assert!(invoke(&["check", "--folder", folder]).status.success(), "{limit}");
    }
    fs::write(&config, format!(r#"{{"{RULE}":-1}}"#)).unwrap();
    let invalid = invoke(&["check", "--folder", folder]);
    assert_eq!(invalid.status.code(), Some(1));
    assert!(String::from_utf8(invalid.stderr).unwrap().contains(RULE));

    let markdown = invoke(&["report", "--folder", folder]);
    assert!(markdown.status.success());
    assert!(String::from_utf8(markdown.stdout).unwrap().contains("| Nested try/catch |"));
}
