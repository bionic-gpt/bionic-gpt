--: Translation()

--! all_translations : Translation
SELECT
    key,
    locale,
    value
FROM
    ops.translations
ORDER BY
    locale,
    key;

--! translations_by_locale : Translation
SELECT
    key,
    locale,
    value
FROM
    ops.translations
WHERE
    locale = :locale
ORDER BY
    key;
