use std::{collections::HashMap, path::Path};

use base::parsers::{interpolated_str::InterpolatedStr, translated::Translated};
use errors::{Errors, ErrorsResult};
use lingual::Lang;

use crate::{
    object::ObjectType,
    parse::{
        file::ParseFile,
        item::{ItemType, MappedObject},
    },
    path::{LocalePaths, ParseInfo, ParsedFileType},
    warn,
    writer::Writers,
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

    let parsed_file = match file_type {
        ParsedFileType::Yaml => ParseFile::open_yaml(path),
        ParsedFileType::Json => ParseFile::open_json(path),
    };

    let dest_files = path.create_locale_files(&langs).await?;
    let mut writers = Writers::from_file_map(dest_files, file_type);
    for (key, value) in parsed_file.into_iter() {
        match value {
            ItemType::String(mut str) => {
                handle_str(&mut str, src_lang, key, &langs, &mut writers).await?;
            }
            ItemType::Object(obj) => {
                handle_object(obj, src_lang, &langs, key, file_type, &mut writers).await?;
            }
        }
    }

    Ok(())
}

async fn handle_str(
    txt: &mut String,
    src_lang: Lang,
    key: String,
    langs: &[Lang],
    writers: &mut Writers,
) -> ErrorsResult<()> {
    let translated_txts = InterpolatedStr::from(txt)
        .translate_bulk(src_lang, langs)
        .await?;

    for translated in translated_txts {
        writers.push(&translated.lang, key.clone(), translated.txt)?;
    }

    Ok(())
}

async fn handle_object(
    map: MappedObject,
    src_lang: Lang,
    langs: &[Lang],
    key: String,
    file_type: ParsedFileType,
    writers: &mut Writers,
) -> ErrorsResult<()> {
    let mut all_translated_items: HashMap<&Lang, ObjectType> = langs
        .iter()
        .map(|lang| (lang, ObjectType::from_file_type(file_type, map.len())))
        .collect();

    for (key, mut value) in map {
        let translated_txts = InterpolatedStr::from(&mut value)
            .translate_bulk(src_lang, langs)
            .await?;
        for translated in translated_txts {
            all_translated_items
                .get_mut(&translated.lang)
                .unwrap()
                .insert(key.clone(), translated.txt);
        }
    }

    for (lang, obj) in all_translated_items {
        // write to file
        writers.push_object(lang, key.clone(), obj)?;
    }

    Ok(())
}

mod test {
    use super::*;
    #[tokio::test]
    async fn translate_test() {
        let props = TranslateProps {
            langs: vec![Lang::En, Lang::Fr],
            src_lang: None,
            file_type: None,
            path: Path::new("../assets/locales/en.yml"),
        };

        translate(props).await.unwrap();
    }
}
