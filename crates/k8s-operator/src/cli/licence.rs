use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use base64::encode;
use ed25519_dalek::{pkcs8::DecodePrivateKey, Signer, SigningKey};
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

pub fn sign(opts: &SignerOpts) -> Result<()> {
    let json = fs::read_to_string(&opts.input)?;
    let key = SigningKey::read_pkcs8_pem_file(&opts.private_key)?;

    let mut value: Value = serde_json::from_str(&json)?;
    if let Some(obj) = value.as_object_mut() {
        obj.remove("signature");
    }
    let data = serde_json::to_vec(&value)?;
    let signature = key.sign(&data);
    let sig_b64 = encode(signature.to_bytes());
    if let Some(obj) = value.as_object_mut() {
        obj.insert("signature".into(), Value::String(sig_b64));
    }
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}
