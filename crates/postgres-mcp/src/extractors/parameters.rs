use serde::de::{self, Deserializer, Error as DeError, SeqAccess, Visitor};
use std::{fmt, marker::PhantomData, str::FromStr};

/// Deserialize a comma separated query parameter into a vector of values using `FromStr`.
pub fn deserialize_comma_separated<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: FromStr,
    T::Err: fmt::Display,
    D: Deserializer<'de>,
{
    struct CommaSeparated<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for CommaSeparated<T>
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a comma separated list")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.trim().is_empty() {
                return Ok(Vec::new());
            }

            v.split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| {
                    T::from_str(item.trim()).map_err(|err| {
                        E::custom(format!(
                            "invalid value `{}` in comma separated list: {err}",
                            item
                        ))
                    })
                })
                .collect()
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut items = Vec::new();
            while let Some(value) = seq.next_element::<String>()? {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                items.push(T::from_str(trimmed).map_err(|err| {
                    DeError::custom(format!("invalid value `{trimmed}` in list: {err}"))
                })?);
            }
            Ok(items)
        }
    }

    deserializer.deserialize_any(CommaSeparated(PhantomData::<T>))
}
