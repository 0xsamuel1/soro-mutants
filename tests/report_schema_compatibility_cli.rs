use std::process::Command;

fn run(command: &str, extra: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args([command, "fixtures/weak-auth"])
        .args(extra)
        .output()
        .expect("CLI should run")
}

#[test]
fn test_json_schema_v1_contract_and_forward_compatibility() {
    let output = run("test", &["--json", "--test-command", "true"]);
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

    // Verify v1 schema envelope
    assert_eq!(report["schema_version"], 1);
    assert!(report["results"].is_array());
    assert!(report["metadata"].is_object());

    // Verify fields of results
    for res in report["results"].as_array().unwrap() {
        let outcome = res["outcome"].as_str().unwrap();
        assert!(matches!(outcome, "KILLED" | "SURVIVED" | "UNVIABLE" | "TIMEOUT"));
        let mutant = &res["mutant"];
        assert!(mutant["id"].is_string());
        assert!(mutant["operator"].is_string());
        assert!(mutant["file"].is_string());
        assert!(mutant["function"].is_string() || mutant["function"].is_null());
        assert!(mutant["span"]["line"].is_number());
        assert!(mutant["span"]["column"].is_number());
        assert!(mutant["original"].is_string());
        assert!(mutant["replacement"].is_string());
        assert!(mutant["description"].is_string());
    }

    // Verify forward-compatibility: consumers ignoring unknown additive fields
    let mut modified = report.clone();
    modified.as_object_mut().unwrap().insert(
        "future_additive_field".to_string(),
        serde_json::json!({"extra": 42}),
    );
    let parsed: cargo_soro_mutants::report::DeJsonTestReport =
        serde_json::from_value(modified).expect("deserializer should ignore additive fields");
    assert_eq!(parsed.schema_version, 1);
}

#[test]
fn legacy_report_fixture_backward_compatibility() {
    let legacy_bytes = include_bytes!("../docs/json-report-example.json");
    let legacy_val: serde_json::Value = serde_json::from_slice(legacy_bytes).unwrap();
    assert_eq!(legacy_val["schema_version"], 1);
    assert!(legacy_val["results"].is_array());

    let parsed: cargo_soro_mutants::report::DeJsonTestReport =
        serde_json::from_slice(legacy_bytes).expect("legacy report should deserialize cleanly");
    assert_eq!(parsed.schema_version, 1);
    assert_eq!(parsed.results.len(), 1);
    assert_eq!(parsed.results[0].mutant.id, "M0001");
}
