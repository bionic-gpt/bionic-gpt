use db::i18n::global;
use db::I18nKey;

pub fn integrations(locale: &str) -> String {
    global().text(locale, I18nKey::Integrations)
}

pub fn integration(locale: &str) -> String {
    global().text(locale, I18nKey::Integration)
}

pub fn datasets(locale: &str) -> String {
    global().text(locale, I18nKey::Datasets)
}

pub fn dataset(locale: &str) -> String {
    global().text(locale, I18nKey::Dataset)
}

pub fn histories(locale: &str) -> String {
    global().text(locale, I18nKey::Histories)
}

pub fn history(locale: &str) -> String {
    global().text(locale, I18nKey::History)
}
