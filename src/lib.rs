#![deny(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod client;
mod error;
mod payment;
mod receipt;

pub use client::*;
pub use error::*;
pub use payment::*;
pub use receipt::*;
