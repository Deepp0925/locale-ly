use std::{collections::HashMap, path::Path};

use base::{parsers::interpolated::InterpolatedStr, pattern::RegexPattern};
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
    /// regex pattern
    pub regex: Option<RegexPattern>,
}

pub async fn translate(props: TranslateProps<'_>) -> ErrorsResult<()> {
    let TranslateProps {
        langs,
        src_lang,
        file_type,
        path,
        regex,
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

    let dest_files = path.create_locale_files(&langs).await?;
    let mut writers = Writers::from_file_map(dest_files, file_type);
    let parsed_file = match file_type {
        ParsedFileType::Yaml => ParseFile::open_yaml(path),
        ParsedFileType::Json => ParseFile::open_json(path),
    };

    for (key, value) in parsed_file.into_iter() {
        match value {
            ItemType::String(mut str) => {
                handle_str(&mut str, src_lang, key, &langs, regex, &mut writers).await?;
            }
            ItemType::Object(obj) => {
                handle_object(obj, src_lang, &langs, key, file_type, regex, &mut writers).await?;
            }
        }
    }

    writers.finish()?;

    path.remove_all_dashes(&langs, file_type).await?;

    Ok(())
}

async fn handle_str(
    txt: &mut String,
    src_lang: Lang,
    key: String,
    langs: &[Lang],
    regex: Option<RegexPattern>,
    writers: &mut Writers,
) -> ErrorsResult<()> {
    // let translated_txts = InterpolatedStr::from_mut_string(txt, regex)
    //     .translate_bulk(src_lang, langs)
    //     .await?;

    // for translated in translated_txts {
    //     writers.push(&translated.lang, key.clone(), translated.txt)?;
    // }

    Ok(())
}

async fn handle_object(
    map: MappedObject,
    src_lang: Lang,
    langs: &[Lang],
    key: String,
    file_type: ParsedFileType,
    regex: Option<RegexPattern>,
    writers: &mut Writers,
) -> ErrorsResult<()> {
    let mut all_translated_items: HashMap<&Lang, ObjectType> = langs
        .iter()
        .map(|lang| (lang, ObjectType::from_file_type(file_type, map.len())))
        .collect();

    for (key, mut value) in map {
        // let translated_txts = InterpolatedStr::from_mut_string(&mut value, regex)
        //     .translate_bulk(src_lang, langs)
        //     .await?;
        // for translated in translated_txts {
        //     all_translated_items
        //         .get_mut(&translated.lang)
        //         .unwrap()
        //         .insert(key.clone(), translated.txt);
        // }
    }

    for (lang, obj) in all_translated_items {
        // write to file
        writers.push_object(lang, key.clone(), obj)?;
    }

    Ok(())
}

mod test {

    #[tokio::test]
    async fn translate_test() {
        use super::*;
        let props = TranslateProps {
            langs: vec![Lang::Fr],
            src_lang: Some(Lang::En),
            file_type: None,
            path: Path::new("../assets/locales/en.yml"),
            regex: Some(RegexPattern::Ruby),
        };

        translate(props).await.unwrap();

        let props = TranslateProps {
            langs: vec![Lang::Fr],
            src_lang: Some(Lang::En),
            file_type: None,
            path: Path::new("../assets/locales/en.json"),
            regex: Some(RegexPattern::Ruby),
        };

        translate(props).await.unwrap();
    }

    #[tokio::test]
    async fn print_lines() {
        use tokio::{
            fs::File,
            io::{AsyncBufReadExt, BufReader},
        };
        let reader = BufReader::new(File::open("../assets/locales/fr.yml").await.unwrap());
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            println!("{}", line);
        }
    }
}
