Internet Acquiring for T-Business clients

[T-Bank docs ](https://developer.tbank.ru/eacq/api)

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
