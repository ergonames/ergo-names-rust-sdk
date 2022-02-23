# Ergo Names Rust SDK

A simple SDK for resolving [Ergo Names](https://ergonames.com).

## Installation

**To install the library:**

A published package will be available once Ergo Names is released on mainnet.

Add this to your Cargo.toml file

```rust
rust-sdk = { git = "https://github.com/ergonames/ergo-names-rust-sdk" }
```

**To import the functions:**

```rust
use ergo_names_rust_sdk;
```

## Documentation

Checking if address exists

```rust
let address = ergo_names_rust_sdk::get_owner_address("bob.ergo");
println!("{}", address);
```

Lookup owner address

```rust
let exists = ergo_names_rust_sdk::check_address_exists("bob.ergo");
println!("{}", exists);
```