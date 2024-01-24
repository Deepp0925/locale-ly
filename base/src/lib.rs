use std::path::Path;

use errors::{Errors, ErrorsResult};
pub use lingual::{Lang, Translator};
use path::{FileType, ParseInfo};
pub use pattern::RegexPattern;
pub mod parser;
mod path;
pub mod pattern;
mod serializers;
use serializers::Writers;

use crate::parser::object::Object;
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("../assets/locales", fallback = "en");

pub(crate) fn warn(str: &str) {
    println!("Warning: {}", str);
}

/// Translates a given file into the given languages
/// # Arguments
/// * `src` - The path to the file to translate
/// * `file_type` - The type of file to translate (yaml/ json/ etc)
/// * `types` - The types of files to write to (yaml/ json/ etc)
/// * `src_lang` - The language of the source file
/// * `langs` - The languages to translate to
/// * `regex` - The regex pattern to use for interpolating string values
/// * `translator` - The translator to use for translating strings to the given languages (see lingual crate)
pub async fn translate_file(
    src: impl AsRef<Path>,
    file_type: Option<FileType>,
    types: &[FileType],
    src_lang: Option<Lang>,
    langs: &[Lang],
    regex: Option<RegexPattern>,
    translator: &Translator,
) -> ErrorsResult<()> {
    if langs.is_empty() {
        warn(&t!("no_langs_specified"));
        return Ok(());
    }

    // check for src lang and file type
    // if not specified then detect using the path
    // if not specified and cannot be detected return Err
    let path_info = src.parse_info();
    let src_lang = src_lang
        .or(path_info.map(|info| info.lang))
        .ok_or(Errors::NoSrcLocale)?;

    let file_type = file_type
        .or(path_info.map(|info| info.file_type))
        .ok_or(Errors::UnknownFileType)?;

    let mut parsed_obj = match file_type {
        FileType::Yaml => Object::open_yaml(&src)?,
        FileType::Json => Object::open_json(&src)?,
    };

    // for ty in types {
    //     if ty == &file_type {
    //         continue;
    //     }

    // }

    let mut serializers = Writers::from_file_types(types, langs, &src).await?;

    for lang in langs {
        let translated_obj = parsed_obj
            .translate_items(regex, translator, &src_lang, lang)
            .await?;

        serializers.write_all(lang, &translated_obj)?;
    }

    Ok(())
}

#[tokio::test]
async fn testing_file() {
    translate_file(
        "../assets/locales/en.yml",
        None,
        &[FileType::Yaml, FileType::Json],
        None,
        &[Lang::Fr, Lang::Es, Lang::De],
        Some(RegexPattern::Ruby),
        &Default::default(),
    )
    .await
    .unwrap();
}
