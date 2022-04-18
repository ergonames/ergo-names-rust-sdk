# Ergo Names Rust SDK

A simple SDK for resolving [Ergo Names](https://ergonames.com).

## Installation

**To install the library:**

A published package will be available once Ergo Names is released on mainnet.

Add this to your Cargo.toml file

```rust
ergo-names-rust-sdk = { git = "https://github.com/ergonames/ergo-names-rust-sdk" }
```

**To import the functions:**

```rust
use ergo_names_rust_sdk;
```

## Documentation

Checking if address exists

```rust
let address = ergo_names_rust_sdk::resolve_ergoname("~balb");
println!("{}", address);
```