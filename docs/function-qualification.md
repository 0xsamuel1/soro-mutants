# Function qualification in reports

Function metadata is qualified when a bare name would be ambiguous across inline modules.

- A top-level function keeps its existing name, for example `transfer`.
- A function inside inline modules is reported with its module path, for example `auth::nested::verify`.
- `--function verify` remains backward-compatible and matches qualified functions by their leaf name.
- A fully qualified filter such as `--function auth::verify` selects only that function.

JSON per-function summaries use the same qualified names as mutant metadata, so downstream consumers can distinguish same-named functions deterministically.

Module qualification does not change mutation semantics or operator recognition.
