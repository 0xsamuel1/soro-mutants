use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("soro-gen-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("generated")).unwrap();
    root
}

#[test]
fn skips_generated_rust_files_via_configurable_patterns() {
    let root = unique_project();
    // Normal hand-written file
    fs::write(
        root.join("src/lib.rs"),
        "pub fn normal(admin: Address) { admin.require_auth(); }\n",
    )
    .unwrap();

    // Generated files that should be skipped when pattern is passed
    fs::write(
        root.join("generated/contract_bindings.rs"),
        "pub fn generated_fn(user: Address) { user.require_auth(); }\n",
    )
    .unwrap();
    fs::write(
        root.join("src/types_generated.rs"),
        "pub fn generated_types(user: Address) { user.require_auth(); }\n",
    )
    .unwrap();

    // 1. Without exclusion: all are discovered
    let all_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--json"])
        .output()
        .expect("CLI should run");
    assert!(all_out.status.success());
    let all_report: serde_json::Value = serde_json::from_slice(&all_out.stdout).unwrap();
    assert_eq!(all_report["mutants"].as_array().unwrap().len(), 3);

    // 2. With explicit exclude-path patterns for generated files
    let filtered_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args([
            "list",
            root.to_str().unwrap(),
            "--exclude-path",
            "generated/**",
            "--exclude-path",
            "*_generated.rs",
            "--json",
        ])
        .output()
        .expect("CLI should run");
    assert!(filtered_out.status.success());
    let filtered_report: serde_json::Value = serde_json::from_slice(&filtered_out.stdout).unwrap();
    let mutants = filtered_report["mutants"].as_array().unwrap();
    assert_eq!(mutants.len(), 1);
    assert_eq!(mutants[0]["file"], "src/lib.rs");

    let _ = fs::remove_dir_all(&root);
}
