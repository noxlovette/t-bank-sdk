#[macro_export]
macro_rules! impl_string_conversions_default {
    ($type:ty) => {
        impl From<String> for $type {
            fn from(value: String) -> Self {
                value.parse().unwrap_or_default()
            }
        }

        impl From<::std::option::Option<String>> for $type {
            fn from(value: ::std::option::Option<String>) -> Self {
                value.map(Into::into).unwrap_or_default()
            }
        }
    };
}
