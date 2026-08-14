//! Proc-macro helpers for libgalaxy-config.
use proc_macro::TokenStream;

#[proc_macro_derive(Config, attributes(config))]
pub fn config_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
