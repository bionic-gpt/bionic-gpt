use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use tokio::sync::RwLock;

use crate::queries;
use crate::Pool;

#[derive(Clone)]
pub struct I18n {
    pool: Pool,
    cache: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

static GLOBAL_I18N: OnceLock<I18n> = OnceLock::new();

impl I18n {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn cached_value(&self, locale: &str, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache
            .get(locale)
            .and_then(|locale_map| locale_map.get(key))
            .cloned()
    }

    async fn populate_locale(&self, locale: &str) -> Result<(), ()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(err) => {
                tracing::error!(target: "db::i18n", ?err, ?locale, "failed to get database client for translations");
                return Err(());
            }
        };

        let rows = match queries::i18n::translations_by_locale()
            .bind(&client, &locale)
            .all()
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(target: "db::i18n", ?err, ?locale, "failed to load translations for locale");
                return Err(());
            }
        };

        drop(client);

        let mut translations = HashMap::with_capacity(rows.len());
        for row in rows {
            translations.insert(row.key, row.value);
        }

        let mut cache = self.cache.write().await;
        cache.insert(locale.to_string(), translations);

        Ok(())
    }

    pub async fn warm_cache(&self) {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(err) => {
                tracing::warn!(target: "db::i18n", ?err, "failed to get database client to warm translations cache");
                return;
            }
        };

        let rows = match queries::i18n::all_translations().bind(&client).all().await {
            Ok(rows) => rows,
            Err(err) => {
                tracing::warn!(target: "db::i18n", ?err, "failed to load translations while warming cache");
                return;
            }
        };

        drop(client);

        let mut grouped: HashMap<String, HashMap<String, String>> = HashMap::new();
        for row in rows {
            grouped
                .entry(row.locale)
                .or_default()
                .insert(row.key, row.value);
        }

        let mut cache = self.cache.write().await;
        for (locale, translations) in grouped {
            cache.insert(locale, translations);
        }
    }

    pub async fn ensure_locale(&self, locale: &str) {
        if self.cache.read().await.contains_key(locale) {
            return;
        }

        if let Err(()) = self.populate_locale(locale).await {
            tracing::warn!(target: "db::i18n", ?locale, "failed to load translations for locale");
        }
    }

    fn cached_value_blocking(&self, locale: &str, key: &str) -> Option<String> {
        self.cache.try_read().ok().and_then(|cache| {
            cache
                .get(locale)
                .and_then(|locale_map| locale_map.get(key))
                .cloned()
        })
    }

    pub fn text(&self, locale: &str, key: I18nKey) -> String {
        let key_str = key.as_str();

        if let Some(value) = self.cached_value_blocking(locale, key_str) {
            return value;
        }

        if locale != "en" {
            if let Some(value) = self.cached_value_blocking("en", key_str) {
                return value;
            }
        }

        key_str.to_string()
    }

    pub async fn t(&self, locale: &str, key: I18nKey) -> String {
        let key_str = key.as_str();

        if let Some(value) = self.cached_value(locale, key_str).await {
            return value;
        }

        if self.populate_locale(locale).await.is_ok() {
            if let Some(value) = self.cached_value(locale, key_str).await {
                return value;
            }
        }

        if locale != "en" {
            if let Some(value) = self.cached_value("en", key_str).await {
                return value;
            }

            if self.populate_locale("en").await.is_ok() {
                if let Some(value) = self.cached_value("en", key_str).await {
                    return value;
                }
            }
        }

        key_str.to_string()
    }
}

pub fn set_global(instance: I18n) {
    if GLOBAL_I18N.set(instance).is_err() {
        tracing::warn!(target: "db::i18n", "global I18n already initialised");
    }
}

pub fn global() -> &'static I18n {
    GLOBAL_I18N
        .get()
        .expect("I18n service has not been initialised")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum I18nKey {
    AiAssistants,
    Integrations,
    Integration,
    Prompts,
    Datasets,
    Assistants,
    Assistant,
    Dataset,
}

impl I18nKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiAssistants => "i18n.ai_assistants",
            Self::Integrations => "i18n.integrations",
            Self::Integration => "i18n.integration",
            Self::Prompts => "i18n.prompts",
            Self::Datasets => "i18n.datasets",
            Self::Assistants => "i18n.assistants",
            Self::Assistant => "i18n.assistant",
            Self::Dataset => "i18n.dataset",
        }
    }
}
