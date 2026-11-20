use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("soro-alias-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn arbitrary_type_aliases_not_inferred_as_soroban_types() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    // Type aliases to primitives or standard types must NOT cause false-positive semantic recognition
    fs::write(
        &src,
        r#"
type AccountId = String;
type UserId = u64;
type NonSdkAddress = (u32, u32);

pub fn process_transfer(user: UserId, account: AccountId, dest: NonSdkAddress) {
    let _ = (user, account, dest);
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
    // No Soroban auth or token mutations should be discovered for non-SDK type aliases
    assert_eq!(mutants.len(), 0);

    let _ = fs::remove_dir_all(&root);
}
