use base::{FileType, RegexPattern};
use clap::Parser;
use lingual::{Lang, Translator};

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("../assets/locales", fallback = "en");

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(author, version, about, long_about = None)]
struct CLIArgs {
    /// The source file to translate
    #[arg(short, long)]
    path: String,
    /// The type of file to translate (yaml/ json/ etc).
    /// If not specified, the file type will be detected from the file extension
    #[arg(long)]
    ty: Option<String>,
    /// The types of files to write to (yaml/ json/ etc) - comma separated
    #[arg(short, long, value_delimiter = ',')]
    formats: Vec<String>,
    /// The language of the source file
    /// If not specified, the language will be detected from the file name
    #[arg(short, long)]
    src_lang: Option<Lang>,
    /// The languages to translate to - comma separated
    #[arg(short, long, value_delimiter = ',')]
    langs: Vec<Lang>,
    /// The regex pattern to use for interpolating string values
    /// If not specified, the default regex pattern will be used (anything between single curly`{..}` braces)
    #[arg(short, long)]
    regex: Option<String>,
    /// The custom pattern to use for interpolating string values
    #[arg(long)]
    pattern: Option<String>,
    /// The translator to use for translating strings to the given languages
    /// If not specified, the default translator will be used (Google Translate)
    /// At the moment, only Google Translate is supported using the free version
    #[arg(short, long)]
    translator: Option<String>,
    /// The api key to use for the translator
    #[arg(short, long)]
    key: Option<String>,
}

fn warn(warn: impl std::fmt::Display) {
    eprintln!("Warning: {}", warn);
}

fn error(err: impl std::fmt::Display) {
    eprintln!("Error: {}", err);
}

pub async fn run() {
    let args = CLIArgs::parse();

    let file_type = args.ty.and_then(|s| FileType::<()>::from_str(&s));
    let formats = args
        .formats
        .iter()
        .filter_map(|s| FileType::<()>::from_str(s))
        .collect::<Vec<_>>();
    let src_lang = args.src_lang;
    let langs = args.langs;

    if langs.is_empty() {
        warn(t!("no_langs_specified"));
        return;
    }

    let pattern = args.pattern;
    // will use the default pattern if the pattern is not specified
    let regex = args
        .regex
        .and_then(|s| RegexPattern::from_str(&s, pattern.as_deref()));

    let translator = match args.translator {
        Some(translator) => match Translator::from_str(&translator, args.key) {
            Some(translator) => translator,
            None => {
                error(t!("no_translator_specified"));
                return;
            }
        },
        None => Default::default(),
    };

    let res = base::translate_file(
        args.path,
        file_type,
        &formats,
        src_lang,
        &langs,
        regex,
        &translator,
    )
    .await;

    if let Err(err) = res {
        error(err.to_str());
    }
}
