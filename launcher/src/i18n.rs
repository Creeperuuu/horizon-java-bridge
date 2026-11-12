//! The launcher currently supports English only. Other language codes fall back to English,
//! including legacy language settings saved by earlier versions.
//!
//! All launcher strings are written directly in English.
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt-br")]
    PortugueseBrazil,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "sl")]
    Slovenian,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "is")]
    Icelandic,
    #[serde(rename = "af")]
    Afrikaans,
    #[serde(rename = "ms")]
    Malay,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "az")]
    Azerbaijani,
    #[serde(rename = "uz")]
    Uzbek,
    #[serde(rename = "kk")]
    Kazakh,
    #[serde(rename = "ka")]
    Georgian,
    #[serde(rename = "hy")]
    Armenian,
    #[serde(rename = "bs")]
    Bosnian,
    #[serde(rename = "mk")]
    Macedonian,
    #[serde(rename = "sq")]
    Albanian,
    #[serde(rename = "cy")]
    Welsh,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "gl")]
    Galician,
    #[serde(rename = "fil")]
    Filipino,
    #[serde(rename = "ta")]
    Tamil,
    #[serde(rename = "mn")]
    Mongolian,
    #[serde(rename = "lo")]
    Lao,
    #[serde(rename = "so")]
    Somali,
    #[serde(rename = "fy")]
    Frisian,
    #[serde(rename = "gd")]
    ScottishGaelic,
    #[serde(rename = "lb")]
    Luxembourgish,
    #[serde(rename = "ky")]
    Kyrgyz,
    #[serde(rename = "kn")]
    Kannada,
    #[serde(rename = "mt")]
    Maltese,
    #[serde(rename = "eo")]
    Esperanto,
    #[serde(rename = "haw")]
    Hawaiian,
    #[serde(rename = "ig")]
    Igbo,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "tt")]
    Tatar,
    #[serde(rename = "yi")]
    Yiddish,
    #[serde(rename = "yo")]
    Yoruba,
    #[serde(rename = "zu")]
    Zulu,
    #[serde(rename = "ba")]
    Bashkir,
    #[serde(rename = "br")]
    Breton,
    #[serde(rename = "cv")]
    Chuvash,
    #[serde(rename = "fo")]
    Faroese,
    #[serde(rename = "fur")]
    Friulian,
    #[serde(rename = "li")]
    Limburgish,
    #[serde(rename = "lmo")]
    Lombard,
    #[serde(rename = "oc")]
    Occitan,
    #[serde(rename = "sah")]
    Sakha,
    #[serde(rename = "se")]
    NorthernSami,
    #[serde(rename = "szl")]
    Silesian,
    #[serde(rename = "vec")]
    Venetian,
}

impl Language {
    pub const ALL: [Language; 1] = [Self::English];

    /// Its code in `settings.json` and on the command line.
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::French => "fr",
            Self::Russian => "ru",
            Self::German => "de",
            Self::Chinese => "zh",
            Self::Spanish => "es",
            Self::Ukrainian => "uk",
            Self::Polish => "pl",
            Self::PortugueseBrazil => "pt-br",
            Self::Japanese => "ja",
            Self::Turkish => "tr",
            Self::Korean => "ko",
            Self::Croatian => "hr",
            Self::Arabic => "ar",
            Self::Hindi => "hi",
            Self::Indonesian => "id",
            Self::Portuguese => "pt",
            Self::Hebrew => "he",
            Self::Hungarian => "hu",
            Self::Romanian => "ro",
            Self::Danish => "da",
            Self::Finnish => "fi",
            Self::Norwegian => "no",
            Self::Bulgarian => "bg",
            Self::Slovak => "sk",
            Self::Lithuanian => "lt",
            Self::Italian => "it",
            Self::Dutch => "nl",
            Self::Vietnamese => "vi",
            Self::Thai => "th",
            Self::Urdu => "ur",
            Self::Czech => "cs",
            Self::Swedish => "sv",
            Self::Greek => "el",
            Self::Persian => "fa",
            Self::Catalan => "ca",
            Self::Estonian => "et",
            Self::Latvian => "lv",
            Self::Slovenian => "sl",
            Self::Serbian => "sr",
            Self::Icelandic => "is",
            Self::Afrikaans => "af",
            Self::Malay => "ms",
            Self::Belarusian => "be",
            Self::Azerbaijani => "az",
            Self::Uzbek => "uz",
            Self::Kazakh => "kk",
            Self::Georgian => "ka",
            Self::Armenian => "hy",
            Self::Bosnian => "bs",
            Self::Macedonian => "mk",
            Self::Albanian => "sq",
            Self::Welsh => "cy",
            Self::Irish => "ga",
            Self::Basque => "eu",
            Self::Galician => "gl",
            Self::Filipino => "fil",
            Self::Tamil => "ta",
            Self::Mongolian => "mn",
            Self::Lao => "lo",
            Self::Somali => "so",
            Self::Frisian => "fy",
            Self::ScottishGaelic => "gd",
            Self::Luxembourgish => "lb",
            Self::Kyrgyz => "ky",
            Self::Kannada => "kn",
            Self::Maltese => "mt",
            Self::Esperanto => "eo",
            Self::Hawaiian => "haw",
            Self::Igbo => "ig",
            Self::Latin => "la",
            Self::Tatar => "tt",
            Self::Yiddish => "yi",
            Self::Yoruba => "yo",
            Self::Zulu => "zu",
            Self::Bashkir => "ba",
            Self::Breton => "br",
            Self::Chuvash => "cv",
            Self::Faroese => "fo",
            Self::Friulian => "fur",
            Self::Limburgish => "li",
            Self::Lombard => "lmo",
            Self::Occitan => "oc",
            Self::Sakha => "sah",
            Self::NorthernSami => "se",
            Self::Silesian => "szl",
            Self::Venetian => "vec",
        }
    }

    /// Its name in itself: what a player looks for in a list of languages.
    pub fn name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::French => "Français",
            Self::Russian => "Русский",
            Self::German => "Deutsch",
            Self::Chinese => "简体中文",
            Self::Spanish => "Español",
            Self::Ukrainian => "Українська",
            Self::Polish => "Polski",
            Self::PortugueseBrazil => "Português (Brasil)",
            Self::Japanese => "日本語",
            Self::Turkish => "Türkçe",
            Self::Korean => "한국어",
            Self::Croatian => "Hrvatski",
            Self::Arabic => "العربية",
            Self::Hindi => "हिन्दी",
            Self::Indonesian => "Bahasa Indonesia",
            Self::Portuguese => "Português (Portugal)",
            Self::Hebrew => "עברית",
            Self::Hungarian => "Magyar",
            Self::Romanian => "Română",
            Self::Danish => "Dansk",
            Self::Finnish => "Suomi",
            Self::Norwegian => "Norsk",
            Self::Bulgarian => "Български",
            Self::Slovak => "Slovenčina",
            Self::Lithuanian => "Lietuvių",
            Self::Italian => "Italiano",
            Self::Dutch => "Nederlands",
            Self::Vietnamese => "Tiếng Việt",
            Self::Thai => "ไทย",
            Self::Urdu => "اردو",
            Self::Czech => "Čeština",
            Self::Swedish => "Svenska",
            Self::Greek => "Ελληνικά",
            Self::Persian => "فارسی",
            Self::Catalan => "Català",
            Self::Estonian => "Eesti",
            Self::Latvian => "Latviešu",
            Self::Slovenian => "Slovenščina",
            Self::Serbian => "Српски",
            Self::Icelandic => "Íslenska",
            Self::Afrikaans => "Afrikaans",
            Self::Malay => "Bahasa Melayu",
            Self::Belarusian => "Беларуская",
            Self::Azerbaijani => "Azərbaycanca",
            Self::Uzbek => "Oʻzbekcha",
            Self::Kazakh => "Қазақша",
            Self::Georgian => "ქართული",
            Self::Armenian => "Հայերեն",
            Self::Bosnian => "Bosanski",
            Self::Macedonian => "Македонски",
            Self::Albanian => "Shqip",
            Self::Welsh => "Cymraeg",
            Self::Irish => "Gaeilge",
            Self::Basque => "Euskara",
            Self::Galician => "Galego",
            Self::Filipino => "Filipino",
            Self::Tamil => "தமிழ்",
            Self::Mongolian => "Монгол",
            Self::Lao => "ລາວ",
            Self::Somali => "Soomaali",
            Self::Frisian => "Frysk",
            Self::ScottishGaelic => "Gàidhlig",
            Self::Luxembourgish => "Lëtzebuergesch",
            Self::Kyrgyz => "Кыргызча",
            Self::Kannada => "ಕನ್ನಡ",
            Self::Maltese => "Malti",
            Self::Esperanto => "Esperanto",
            Self::Hawaiian => "ʻŌlelo Hawaiʻi",
            Self::Igbo => "Igbo",
            Self::Latin => "Latina",
            Self::Tatar => "Татарча",
            Self::Yiddish => "ייִדיש",
            Self::Yoruba => "Yorùbá",
            Self::Zulu => "isiZulu",
            Self::Bashkir => "Башҡортса",
            Self::Breton => "Brezhoneg",
            Self::Chuvash => "Чӑвашла",
            Self::Faroese => "Føroyskt",
            Self::Friulian => "Furlan",
            Self::Limburgish => "Limburgs",
            Self::Lombard => "Lombard",
            Self::Occitan => "Occitan",
            Self::Sakha => "Саха тыла",
            Self::NorthernSami => "Davvisámegiella",
            Self::Silesian => "Ślōnskŏ",
            Self::Venetian => "Vèneto",
        }
    }

    /// Parses a language code (e.g. `fr`, `fr-CH`, `pt-BR`, or `pt_BR.UTF-8`)
    /// if the launcher supports it.
    ///
    /// Underscores are normalized to hyphens to support locale formats commonly
    /// returned by operating systems. Brazilian Portuguese (`pt-BR`) is matched
    /// explicitly, while other Portuguese variants (such as `pt-PT`) are not
    /// mapped to it.
    pub fn parse(code: &str) -> Option<Self> {
        let lower = code.trim().to_ascii_lowercase();
        let clean = lower.split(['.', '@']).next().unwrap_or("");
        let normalized = clean.replace('_', "-");

        if normalized == "pt-br" {
            return Some(Self::PortugueseBrazil);
        }

        let base = match normalized.split('-').next().unwrap_or("") {
            // Systems name Norwegian by its written form: Bokmål (`nb-NO`, what macOS and Windows
            // report) or Nynorsk (`nn`). The launcher's one table is Bokmål.
            "nb" | "nn" => "no",
            base => base,
        };

        Self::ALL
            .into_iter()
            .find(|language| language.code() == base)
    }

    /// The first of the system's preferred languages the launcher has, else English.
    pub fn pick<'a>(preferred: impl IntoIterator<Item = &'a str>) -> Self {
        preferred
            .into_iter()
            .find_map(Self::parse)
            .unwrap_or_default()
    }

    /// The system's language, as far as the launcher has it.
    pub fn system() -> Self {
        let locales: Vec<String> = sys_locale::get_locales().collect();
        Self::pick(locales.iter().map(String::as_str))
    }


}

static CURRENT: AtomicU8 = AtomicU8::new(0);

/// Shows the launcher in `language` from now on.
pub fn set(language: Language) {
    let i = Language::ALL
        .iter()
        .position(|l| *l == language)
        .unwrap_or(0);
    CURRENT.store(i as u8, Ordering::Relaxed);
}

pub fn current() -> Language {
    Language::ALL[CURRENT.load(Ordering::Relaxed) as usize % Language::ALL.len()]
}

/// English is the only supported launcher language. Legacy language settings still
/// resolve to English rather than requiring their old translation files.
pub fn tr_in(_language: Language, en: &str) -> &str {
    en
}

/// `en` in the launcher's language.
pub fn tr(en: &str) -> &str {
    en
}

/// `en` in the launcher's language with its `{name}` holes filled.
pub fn trf(en: &str, args: &[(&str, &str)]) -> String {
    let mut out = tr(en).to_string();
    for (name, value) in args {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out
}

#[cfg(test)]
#[path = "i18n/tests.rs"]
mod tests;
