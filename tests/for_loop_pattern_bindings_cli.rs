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
        std::env::temp_dir().join(format!("soro-forloop-test-{}-{nonce}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    root
}

#[test]
fn respects_for_loop_pattern_bindings_in_address_scope() {
    let root = unique_project();
    let src = root.join("src/lib.rs");
    fs::write(
        &src,
        r#"
pub fn loop_process(admin: Address, items: soroban_sdk::Vec<Address>) {
    for item in items.iter() {
        admin.require_auth();
        item.require_auth();
    }

    admin.require_auth();
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
        "for loops should parse and respect loop bindings"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mutants = report["mutants"].as_array().unwrap();
    let item_substitutions: Vec<_> = mutants
        .iter()
        .filter(|m| {
            m["operator"] == "AUTH-002"
                && m["original"] == "admin.require_auth()"
                && m["replacement"] == "item.require_auth()"
        })
        .collect();
    assert_eq!(
        item_substitutions.len(),
        1,
        "the loop-local Address binding must be available only inside the loop body"
    );
    assert!(
        item_substitutions[0]["span"]["line"].as_u64().unwrap() < 8,
        "the post-loop admin authorization must not see the loop-local item binding"
    );

    let _ = fs::remove_dir_all(&root);
}
