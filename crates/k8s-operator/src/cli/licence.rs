use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use base64::encode;
use ed25519_dalek::{pkcs8::DecodePrivateKey, Signer, SigningKey};
use serde::Serialize;
use serde_json::Value;

#[derive(clap::Parser)]
pub struct SignerOpts {
    /// Path to the licence JSON file
    #[arg(long)]
    pub input: PathBuf,
    /// Path to the private key encoded in PKCS8 DER
    #[arg(long)]
    pub private_key: PathBuf,
}

#[derive(Serialize)]
struct SignableLicence {
    user_count: usize,
    hostname_url: String,
    end_date: String,
    app_name: String,
    app_logo_svg: String,
}

pub fn sign(opts: &SignerOpts) -> Result<()> {
    let json = fs::read_to_string(&opts.input)?;
    let key = SigningKey::read_pkcs8_pem_file(&opts.private_key)?;

    let mut value: Value = serde_json::from_str(&json)?;
    {
        let Some(obj) = value.as_object_mut() else {
            return Err(anyhow!("LICENCE JSON must be an object"));
        };

        obj.remove("signature");

        let user_count = obj
            .get("user_count")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("LICENCE JSON missing numeric user_count"))?
            as usize;
        let hostname_url = obj
            .get("hostname_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("LICENCE JSON missing hostname_url"))?
            .to_string();
        let end_date = obj
            .get("end_date")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("LICENCE JSON missing end_date"))?
            .to_string();
        let app_name = obj
            .get("app_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("LICENCE JSON missing app_name"))?
            .to_string();
        let app_logo_svg = obj
            .get("app_logo_svg")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("LICENCE JSON missing app_logo_svg"))?
            .to_string();

        let canonical = SignableLicence {
            user_count,
            hostname_url,
            end_date,
            app_name,
            app_logo_svg,
        };

        let data = serde_json::to_vec(&canonical)?;
        let signature = key.sign(&data);
        let sig_b64 = encode(signature.to_bytes());

        obj.insert("signature".into(), Value::String(sig_b64));
    }

    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}
