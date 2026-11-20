# Reliable integration tests

Integration tests should be deterministic and isolated so that a failure signals a product regression rather than timing or shared-state interference.

## Rules

- Prefer synchronous child-process completion (`Command::output`) to arbitrary `sleep` calls or polling based on elapsed wall time.
- Give each test exclusive temporary paths; avoid fixed global scratch directories, shared report filenames, and persistent state between tests. Use a unique temporary directory and clean it up on success.
- Pass CLI arguments separately with `Command::arg` rather than constructing a shell command string. This also handles paths containing spaces.
- Assert process exit status and include stderr when a subprocess fails. Parse JSON reports structurally and compare byte-for-byte only for output explicitly promised to be deterministic.
- Keep test inputs fixed and local. Do not depend on network services, external benchmark repositories, arbitrary clocks, or execution order.
- For runner tests, verify both normal cleanup and failure paths; do not classify compile-invalid mutants as killed.
- Keep regression targets bounded in CI and avoid retries that hide nondeterministic failures.

## Regression seed and reproduction

`tests/flaky_resistance_cli.rs` launches four independent concurrent CLI discovery processes against the same immutable `fixtures/weak-auth` fixture. It checks that every process succeeds and emits the same JSON. This detects shared mutable scratch-state regressions without introducing a race-prone timeout or a sleep.

Run locally:

```bash
cargo test --locked --test flaky_resistance_cli -- --nocapture
cargo test --locked --test flaky_resistance_cli -- --test-threads=1
```

The [integration determinism workflow](../.github/workflows/integration-determinism.yml) runs both commands on pull requests with a 12-minute job bound. A failure should include the child process stderr and a minimal reproducer; don't resolve it by adding a sleep or blindly retrying the job. This test does not alter mutation semantics, reporting schemas, or the CLI.
