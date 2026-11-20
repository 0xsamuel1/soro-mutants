# JSON summary breakdowns

Successful `test --json` reports include three additive summary objects alongside `results` and `metadata`:

- `by_operator` groups generated, killed, survived, unviable, and timeout counts by operator ID.
- `by_file` groups the same counts by project-relative source file.
- `by_function` groups counts by function name when function metadata is available.

Keys are emitted in deterministic sorted order. Empty result sets produce empty objects. These fields are additive to schema version 1, so existing consumers that ignore unknown fields remain compatible.

Example:

```json
{
  "by_operator": {
    "AUTH-001": {
      "generated": 2,
      "killed": 1,
      "survived": 1,
      "unviable": 0,
      "timeout": 0
    }
  }
}
```
