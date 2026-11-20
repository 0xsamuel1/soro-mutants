# Control-flow binding scope

Soro Mutants keeps semantic discovery conservative around Rust control-flow patterns.

- Calls inside `if let` and `while let` bodies are visited without leaking pattern bindings outside their lexical scope.
- For `for binding in addresses.iter()`, when `addresses` is an explicitly typed `Vec<Address>` function parameter and the loop pattern is a simple identifier, the loop-local binding is treated as an in-scope Address candidate only for the loop body. This allows authorization substitutions such as `admin.require_auth()` → `item.require_auth()` while preventing the loop-local name from leaking after the loop.
- Tuple and struct destructuring whose element types cannot be proven from the syntax are intentionally not guessed. Their method calls may be ignored by type-sensitive operators rather than creating false-positive substitutions.

The analysis does not infer arbitrary iterator item types, follow aliases through user-defined containers, or broaden Soroban type recognition beyond explicit supported shapes. This keeps discovery deterministic and avoids unsafe semantic guesses.
