use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("soro-assoc-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn filters_associated_functions_and_methods_in_impl_blocks_without_collision() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    fs::write(
        &src,
        r#"
pub struct Vault;

impl Vault {
    pub fn new_vault(owner: Address) {
        owner.require_auth();
    }

    pub fn deposit(&self, depositor: Address) {
        depositor.require_auth();
    }
}
"#,
    )
    .unwrap();

    // 1. Filter associated function `new_vault`
    let assoc_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--function", "new_vault", "--json"])
        .output()
        .expect("CLI should run");
    assert!(assoc_out.status.success());
    let assoc_report: serde_json::Value = serde_json::from_slice(&assoc_out.stdout).unwrap();
    let assoc_mutants = assoc_report["mutants"].as_array().unwrap();
    assert_eq!(assoc_mutants.len(), 1);
    assert_eq!(assoc_mutants[0]["function"], "new_vault");

    // 2. Filter method `deposit`
    let method_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--function", "deposit", "--json"])
        .output()
        .expect("CLI should run");
    assert!(method_out.status.success());
    let method_report: serde_json::Value = serde_json::from_slice(&method_out.stdout).unwrap();
    let method_mutants = method_report["mutants"].as_array().unwrap();
    assert_eq!(method_mutants.len(), 1);
    assert_eq!(method_mutants[0]["function"], "deposit");

    let _ = fs::remove_dir_all(&root);
}
