# Modern Rust discovery and exclusions

Soro Mutants keeps source discovery conservative while supporting modern stable Rust syntax.

## Generated files

Generated Rust is not skipped implicitly. Use repeatable `--exclude-path` globs when generated sources should be outside the mutation surface:

```text
cargo soro-mutants list . --exclude-path 'generated/**' --exclude-path '*_generated.rs'
```

Patterns are project-root-relative and exclusions win over includes. Files that do not match an exclusion remain eligible for normal discovery.

## Associated functions and methods

`--function` applies to functions declared in `impl` blocks as well as free functions. A leaf-name filter such as `--function deposit` remains supported. When report metadata is qualified to disambiguate an impl or module, the corresponding qualified name may also be used.

## Const blocks

Stable Rust `const { ... }` blocks are parsed without inventing Soroban semantic types for ordinary compile-time expressions. Supported calls outside the const block remain discoverable; unrelated expressions inside the block do not create semantic mutants.

## Let-else

`let PATTERN = EXPR else { ... };` is accepted as ordinary stable Rust syntax. The divergent `else` block and the following scope are both visited deterministically.

Type-sensitive mutation operators remain conservative: a binding is treated as a Soroban value only when its supported type can be proven from the surrounding syntax. Unsupported pattern inference is ignored rather than guessed, avoiding false-positive substitutions.

These rules apply consistently to both `list` and `test`.
