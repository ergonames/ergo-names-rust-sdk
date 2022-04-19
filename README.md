# Ergo Names Rust SDK

A simple SDK for resolving [Ergo Names](https://ergonames.com).

## Installation

**To install the library:**

A published package will be available once Ergo Names is released on mainnet.

Add this to your Cargo.toml file

```rust
ergonames-rust = "0.1.0"
```

**To import the functions:**

```rust
use ergonames_rust;
```

## Documentation

Checking if address exists

```rust
let address = ergonames_rust::resolve_ergoname("~balb");
println!("{}", address);
```