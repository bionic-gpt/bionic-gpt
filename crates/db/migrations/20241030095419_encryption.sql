-- migrate:up
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Encrypt function
CREATE OR REPLACE FUNCTION encrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    encrypted text;
BEGIN
    -- Retrieve the 'app.root_key' parameter, allowing it to be missing (returns NULL if not set)
    key := current_setting('encryption.root_key', true);
    
    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            -- Encrypt the data using pgp_sym_encrypt with 'armor' option
            encrypted := pgp_sym_encrypt(data, key, 'compress-algo=1, cipher-algo=aes256');
            RETURN encrypted;
        EXCEPTION WHEN others THEN
            -- Return the exception message if encryption fails
            RETURN SQLERRM;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;


-- Decrypt function

CREATE OR REPLACE FUNCTION decrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    decrypted text;
BEGIN
    -- Retrieve the 'app.root_key' parameter, allowing it to be missing (returns NULL if not set)
    key := current_setting('encryption.root_key', true);
    
    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            -- Decrypt the data using pgp_sym_decrypt
            decrypted := pgp_sym_decrypt(data::bytea, key);
            RETURN decrypted;
        EXCEPTION WHEN others THEN
            -- Return the exception message if decryption fails
            RETURN data;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- migrate:down

