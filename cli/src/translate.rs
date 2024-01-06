use std::path::Path;

use base::parsers::interpolated_str::InterpolatedStr;
use errors::{Errors, ErrorsResult};
use lingual::Lang;

use crate::{
    parse::{file::ParseFile, item::ItemType},
    path::{GenLocalePaths, ParseInfo, ParsedFileType},
    warn,
};

pub struct TranslateProps<'a> {
    /// the langs to translate to
    pub langs: Vec<Lang>,
    /// Will detect if not specified
    pub src_lang: Option<Lang>,
    /// will detect if not specified
    pub file_type: Option<ParsedFileType>,
    pub path: &'a Path,
}

pub async fn translate(props: TranslateProps<'_>) -> ErrorsResult<()> {
    let TranslateProps {
        langs,
        src_lang,
        file_type,
        path,
    } = props;

    if langs.is_empty() {
        warn(&t!("no_langs_specified"));
        return Ok(());
    }

    // check for src lang and file type
    // if not specified then detect using the path
    // if not specified and cannot be detected return Err
    let path_info = path.parse_info();
    let src_lang = src_lang
        .or(path_info.map(|info| info.lang))
        .ok_or(Errors::NoSrcLocale)?;

    let file_type = file_type
        .or(path_info.map(|info| info.file_type))
        .ok_or(Errors::UnknownFileType)?;

    let mut parsed_file = match file_type {
        ParsedFileType::Yaml => ParseFile::open_yaml(path),
        ParsedFileType::Json => ParseFile::open_json(path),
    };

    let dest_files = path.create_locale_files(&langs).await;

    for (key, value) in parsed_file.into_iter() {
        let translated_txts = match value {
            ItemType::String(txt) => {
                let translated = InterpolatedStr::from(txt)
                    .translate_bulk(src_lang, &langs)
                    .await?;
            }
            ItemType::Object(map) => todo!(),
        };
    }
    Ok(())
}
