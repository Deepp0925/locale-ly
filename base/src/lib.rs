pub mod file;
pub mod parsers;
pub mod pattern;
mod serializers;
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("../assets/locales", fallback = "en");
