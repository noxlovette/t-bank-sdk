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
mod token;

pub use client::*;
pub use error::*;
pub use payment::*;
pub use receipt::*;

#[macro_export]
macro_rules! newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident ($ty:ty);) => {
        $(#[$meta])*
        #[derive(::serde::Serialize, ::serde::Deserialize, ::core::fmt::Debug)]
        #[serde(transparent)]
        $vis struct $name($ty);

        impl ::std::fmt::Display for $name
        where
            $ty: ::std::fmt::Display,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}
