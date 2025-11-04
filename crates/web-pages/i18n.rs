use db::i18n::global;
use db::I18nKey;

pub fn ai_assistants(locale: &str) -> String {
    global().text(locale, I18nKey::AiAssistants)
}

pub fn integrations(locale: &str) -> String {
    global().text(locale, I18nKey::Integrations)
}

pub fn integration(locale: &str) -> String {
    global().text(locale, I18nKey::Integration)
}

pub fn prompts(locale: &str) -> String {
    global().text(locale, I18nKey::Prompts)
}

pub fn datasets(locale: &str) -> String {
    global().text(locale, I18nKey::Datasets)
}

pub fn assistants(locale: &str) -> String {
    global().text(locale, I18nKey::Assistants)
}

pub fn assistant(locale: &str) -> String {
    global().text(locale, I18nKey::Assistant)
}

pub fn dataset(locale: &str) -> String {
    global().text(locale, I18nKey::Dataset)
}
