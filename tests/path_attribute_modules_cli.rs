use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("soro-path-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src/custom_sub")).unwrap();
    root
}

#[test]
fn recognizes_module_declared_with_path_attribute() {
    let root = unique_project();
    fs::write(
        root.join("src/lib.rs"),
        r#"
#[path = "custom_sub/auth_impl.rs"]
mod custom_auth;
"#,
    )
    .unwrap();

    fs::write(
        root.join("src/custom_sub/auth_impl.rs"),
        r#"
pub fn check_authority(admin: Address) {
    admin.require_auth();
}
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--json"])
        .output()
        .expect("CLI should run");
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mutants = report["mutants"].as_array().unwrap();
    assert!(
        mutants.iter().any(|m| m["file"].as_str().unwrap().contains("auth_impl.rs")),
        "mutant in #[path] module file must be recognized"
    );

    let _ = fs::remove_dir_all(&root);
}
