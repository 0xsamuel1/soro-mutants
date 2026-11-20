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
        std::env::temp_dir().join(format!("soro-mod-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

fn list(root: &std::path::Path, function: Option<&str>) -> serde_json::Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"));
    command.arg("list").arg(root);
    if let Some(function) = function {
        command.args(["--function", function]);
    }
    let output = command.arg("--json").output().expect("CLI should run");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn nested_inline_modules_filter_and_disambiguate_correctly() {
    let root = unique_project();
    fs::write(
        root.join("src/lib.rs"),
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

    let report = list(&root, None);
    let mutants = report["mutants"].as_array().unwrap();
    let functions: Vec<_> = mutants
        .iter()
        .map(|mutant| mutant["function"].as_str().unwrap())
        .collect();
    assert_eq!(
        functions,
        vec![
            "auth_primary::verify",
            "auth_secondary::verify",
            "auth_secondary::nested_inner::verify",
        ]
    );

    let leaf = list(&root, Some("verify"));
    assert_eq!(leaf["mutants"].as_array().unwrap().len(), 3);

    let qualified = list(&root, Some("auth_primary::verify"));
    let qualified_mutants = qualified["mutants"].as_array().unwrap();
    assert_eq!(qualified_mutants.len(), 1);
    assert_eq!(
        qualified_mutants[0]["function"],
        serde_json::Value::String("auth_primary::verify".into())
    );

    let test_output = Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .args([
            "test",
            root.to_str().unwrap(),
            "--test-command",
            "true",
            "--json",
        ])
        .output()
        .expect("test CLI should run");
    assert!(
        test_output.status.success(),
        "{}",
        String::from_utf8_lossy(&test_output.stderr)
    );
    let test_report: serde_json::Value = serde_json::from_slice(&test_output.stdout).unwrap();
    assert_eq!(
        test_report["by_function"]["auth_primary::verify"]["generated"],
        1
    );
    assert_eq!(
        test_report["by_function"]["auth_secondary::nested_inner::verify"]["generated"],
        1
    );

    let _ = fs::remove_dir_all(&root);
}
