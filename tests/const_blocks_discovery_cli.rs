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
        std::env::temp_dir().join(format!("soro-const-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn avoids_false_positive_mutations_in_const_blocks() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    fs::write(
        &src,
        r#"
pub fn check_with_const(admin: Address) {
    admin.require_auth();

    // const block expression
    const {
        let x = 10;
        let _ = x * 2;
    };
}
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--json"])
        .output()
        .expect("CLI should run");
    assert!(
        output.status.success(),
        "const block syntax must parse without error"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mutants = report["mutants"].as_array().unwrap();
    // Only admin.require_auth() should be mutated; no false positives inside const block
    assert_eq!(mutants.len(), 1);
    assert_eq!(mutants[0]["operator"], "AUTH-001");

    let _ = fs::remove_dir_all(&root);
}
