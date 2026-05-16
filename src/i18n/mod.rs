mod de;
mod en;
mod zh;

use crate::Language;

pub fn t(lang: Language, key: &str) -> &'static str {
    match lang {
        Language::English => en::t(key),
        Language::German => de::t(key),
        Language::Chinese => zh::t(key),
    }
}
