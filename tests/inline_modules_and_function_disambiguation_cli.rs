use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn unique_project() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("soro-mod-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn nested_inline_modules_filter_and_disambiguate_correctly() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    fs::write(
        &src,
        r#"
mod auth_primary {
    pub fn verify(user: Address) {
        user.require_auth();
    }
}

mod auth_secondary {
    pub fn verify(user: Address) {
        user.require_auth();
    }

    pub mod nested_inner {
        pub fn verify(user: Address) {
            user.require_auth();
        }
    }
}
"#,
    )
    .unwrap();

    // 1. List all mutants with --json
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--json"])
        .output()
        .expect("CLI should run");
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mutants = report["mutants"].as_array().unwrap();
    assert_eq!(mutants.len(), 3);

    // 2. Filter by function name
    let filtered_out = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args(["list", root.to_str().unwrap(), "--function", "verify", "--json"])
        .output()
        .expect("CLI should run");
    assert!(filtered_out.status.success());
    let filtered_report: serde_json::Value = serde_json::from_slice(&filtered_out.stdout).unwrap();
    assert_eq!(filtered_report["mutants"].as_array().unwrap().len(), 3);

    let _ = fs::remove_dir_all(&root);
}
