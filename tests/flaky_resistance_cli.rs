//! Deterministic, isolated CLI integration tests without sleeps or shared scratch state.
use serde_json::Value;
use std::process::Command;

fn discover() -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-soro-mutants"))
        .arg("list")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/weak-auth"))
        .arg("--json")
        .output()
        .expect("launch list command")
}

#[test]
fn concurrent_discovery_does_not_share_mutable_state() {
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(discover))
        .collect();
    let outputs: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().expect("discovery worker panicked"))
        .collect();
    let first = &outputs[0];
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let parsed: Value = serde_json::from_slice(&first.stdout).expect("valid discovery JSON");
    assert!(!parsed.is_null(), "discovery should produce structured JSON");
    for output in outputs.iter().skip(1) {
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stdout, first.stdout,
            "independent concurrent CLI invocations must produce identical reports"
        );
    }
}
