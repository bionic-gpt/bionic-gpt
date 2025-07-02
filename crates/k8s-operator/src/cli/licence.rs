use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use base64::{decode, encode};
use ed25519_dalek::{Keypair, Signer};
use serde_json::Value;

#[derive(clap::Parser)]
pub struct SignerOpts {
    /// Path to the licence JSON file
    #[arg(long)]
    pub input: PathBuf,
    /// Path to the private key encoded in base64
    #[arg(long)]
    pub private_key: PathBuf,
}

pub fn sign(opts: &SignerOpts) -> Result<()> {
    let json = fs::read_to_string(&opts.input)?;
    let key_b64 = fs::read_to_string(&opts.private_key)?;
    let key_bytes = decode(key_b64.trim())?;
    let keypair = Keypair::from_bytes(&key_bytes)?;

    let mut value: Value = serde_json::from_str(&json)?;
    if let Some(obj) = value.as_object_mut() {
        obj.remove("signature");
    }
    let data = serde_json::to_vec(&value)?;
    let signature = keypair.sign(&data);
    let sig_b64 = encode(signature.to_bytes());
    if let Some(obj) = value.as_object_mut() {
        obj.insert("signature".into(), Value::String(sig_b64));
    }
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
