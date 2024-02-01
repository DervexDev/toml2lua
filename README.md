# toml2lua
Convert TOML to Lua table

<div>
  <a href="https://crates.io/crates/toml2lua"><img alt='Version badge' src='https://img.shields.io/crates/v/toml2lua.svg'></a>
  <a href="https://crates.io/crates/toml2lua"><img alt='Downloads badge' src='https://img.shields.io/crates/d/toml2lua.svg'></a>
  <a href="https://crates.io/crates/toml2lua"><img alt='License badge' src='https://img.shields.io/crates/l/toml2lua.svg'></a>
  <a href="https://docs.rs/toml2lua"><img alt="Docs badge" src="https://img.shields.io/docsrs/toml2lua"></a>
</div>

## Example:
```rust
use toml2lua::parse;

let toml = r#"
string = "abc"
int = 123
bool = true

[object]
key = "value"
"#;

let lua = parse(toml).unwrap();
// Output:
// {
//   ["string"] = "abc",
//   ["int"] = 123,
//   ["bool"] = true,
//   ["object"] = {
//     ["key"] = "value",
//   },
// }
```
