# t-bank-sdk

[![Crates.io](https://img.shields.io/crates/v/t-bank-sdk.svg)](https://crates.io/crates/t-bank-sdk)
[![docs.rs](https://img.shields.io/docsrs/t-bank-sdk)](https://docs.rs/t-bank-sdk)
[![CI](https://github.com/noxlovette/t-bank-sdk/actions/workflows/rust.yml/badge.svg)](https://github.com/noxlovette/t-bank-sdk/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Async Rust SDK for [T-Bank's](https://www.tbank.ru/) (T-Business) Internet Acquiring API.

[T-Bank API docs](https://developer.tbank.ru/eacq/api)

## Features

- Full payment lifecycle: init, confirm, cancel/refund, charge (recurrent), get state
- Customer & saved-card management (add/remove customer, add/remove/list cards)
- Receipts (54-FZ) and marketplace `Shops` splits
- Request signing (token derivation) handled for you
- Built-in trust anchors for T-Bank's Russian state PKI certificate chain
- Optional `serde` (de)serialization and `utoipa` OpenAPI schema support

## Installation

```sh
cargo add t-bank-sdk
```

Enable optional features as needed:

```toml
[dependencies]
t-bank-sdk = { version = "0.3", features = ["serde"] }
```

| Feature  | Enables                                                        |
| -------- | --------------------------------------------------------------- |
| `serde`  | `Serialize`/`Deserialize` on request/response types            |
| `utoipa` | `utoipa::ToSchema` on request/response types, for OpenAPI docs |

## Credentials modes

### 1) Central mode (default)

`Client::new()` reads:

- `TBANK_ENV` (`Test` / `Production`)
- `TERMINAL_ID`
- `TBANK_PASSWORD`

and stores credentials in the client.

```rust
use t_bank_sdk::{Client, InitPaymentReq};

let client = Client::new().await?;
let payload = InitPaymentReq::new(
    client.terminal_key(),
    1000,
    "order-1",
);
let response = client.initiate_payment(payload).await?;
```

### 2) External mode

`Client::external()` reads only `TBANK_ENV`.
Credentials are passed at call-time:

```rust
use t_bank_sdk::{Client, InitPaymentReq, Password, TerminalKey};

let client = Client::external().await?;
let terminal_key = TerminalKey::new("TBankTest")?;
let password = Password::new("secret")?;
let payload = InitPaymentReq::new(&terminal_key, 1000, "order-1");

let response = client
    .initiate_payment_with_credentials(payload, &terminal_key, &password)
    .await?;
```

## License

Licensed under the [MIT license](LICENSE).
