# Ergo Names Rust SDK

A simple SDK for resolving [Ergo Names](https://ergonames.com).

### Example

```rust
let name: String = "~balb".to_owned();
let address: String = ergonames::resolve_ergoname(name);
```