use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("soro-letelse-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn handles_let_else_syntax_around_soroban_values() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    fs::write(
        &src,
        r#"
pub fn guarded_operation(target: Option<Address>, fallback: Address) {
    let Some(user) = target else {
        fallback.require_auth();
        return;
    };

    user.require_auth();
}
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--json"])
        .output()
        .expect("CLI should run");
    assert!(output.status.success(), "let-else syntax must parse and discover mutants");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mutants = report["mutants"].as_array().unwrap();
    // Mutants inside fallback divergence and following let-else should be discovered
    assert!(!mutants.is_empty());
    assert!(mutants.iter().any(|m| m["operator"] == "AUTH-001"));

    let _ = fs::remove_dir_all(&root);
}
