use std::{fs, process::Command};

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(args)
        .output()
        .expect("CLI should run")
}

#[test]
fn zero_mutant_reports_across_all_formats() {
    let target = "fixtures/weak-auth";
    let non_matching_op = "NONEXISTENT-OPERATOR";

    // 1. Text mode
    let text_out = run(&["test", target, "--operator", non_matching_op]);
    assert!(text_out.status.success());
    let text = String::from_utf8_lossy(&text_out.stdout);
    assert!(text.contains("No matching Soroban semantic mutants found."));

    // 2. JSON mode
    let json_out = run(&["test", target, "--operator", non_matching_op, "--json"]);
    assert!(json_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&json_out.stdout).unwrap();
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["results"], serde_json::json!([]));
    assert!(json["metadata"].is_object());

    // 3. Markdown mode
    let md_out = run(&["test", target, "--operator", non_matching_op, "--markdown"]);
    assert!(md_out.status.success());
    let md = String::from_utf8_lossy(&md_out.stdout);
    assert!(md.contains("# Soro Mutants Report"));
    assert!(md.contains("Semantic score: 0.0%"));
    assert!(md.contains("| Generated | 0 |"));
    assert!(md.contains("_None._"));

    // 4. HTML mode
    let html_out = run(&["test", target, "--operator", non_matching_op, "--html"]);
    assert!(html_out.status.success());
    let html = String::from_utf8_lossy(&html_out.stdout);
    assert!(html.contains("<!doctype html>"));
    assert!(html.contains("Semantic score: 0.0%"));
    assert!(html.contains("<h2>Mutant details</h2>"));

    // 5. SARIF mode
    let sarif_out = run(&["test", target, "--operator", non_matching_op, "--sarif"]);
    assert!(sarif_out.status.success());
    let sarif: serde_json::Value = serde_json::from_slice(&sarif_out.stdout).unwrap();
    assert_eq!(sarif["version"], "2.1.0");
    assert_eq!(sarif["runs"][0]["results"].as_array().unwrap().len(), 0);
    assert_eq!(sarif["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap().len(), 0);

    // 6. GitHub step summary mode
    let temp_dir = std::env::temp_dir();
    let summary_file = temp_dir.join(format!("soro-summary-test-{}", std::process::id()));
    let gh_summary_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(&["test", target, "--operator", non_matching_op, "--github-summary"])
        .env("GITHUB_STEP_SUMMARY", &summary_file)
        .output()
        .expect("CLI should run");
    assert!(gh_summary_out.status.success());
    let summary_content = fs::read_to_string(&summary_file).expect("summary file should be written");
    let _ = fs::remove_file(&summary_file);
    assert!(summary_content.contains("# Soro Mutants Report"));
    assert!(summary_content.contains("Semantic score: 0.0%"));
}

#[test]
fn zero_mutant_list_reports() {
    let target = "fixtures/weak-auth";
    let non_matching_op = "NONEXISTENT-OPERATOR";

    let text_out = run(&["list", target, "--operator", non_matching_op]);
    assert!(text_out.status.success());
    assert!(String::from_utf8_lossy(&text_out.stdout).contains("No matching Soroban semantic mutants found."));

    let json_out = run(&["list", target, "--operator", non_matching_op, "--json"]);
    assert!(json_out.status.success());
    let json: serde_json::Value = serde_json::from_slice(&json_out.stdout).unwrap();
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["mutants"], serde_json::json!([]));
}
