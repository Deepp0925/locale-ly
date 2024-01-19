// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("../assets/locales", fallback = "en");

// mod object;
// mod parse;
// mod path;
// mod translate;
// mod writer;

pub(crate) fn warn(str: &str) {
    println!("Warning: {}", str);
}
