-- migrate:up
INSERT INTO translations (locale, key, value) VALUES
    ('en', 'i18n.histories', 'Chat History'),
    ('en', 'i18n.history', 'Chat History');

-- migrate:down
DELETE FROM translations WHERE key IN ('i18n.histories', 'i18n.history') AND locale = 'en';
