use std::path::PathBuf;

use hf_hub::api::sync::ApiBuilder;

fn main() {
    //let api = Api::new().unwrap();
    let api = ApiBuilder::new()
        .with_progress(false)
        .with_cache_dir(PathBuf::from(r"/tmp"))
        .build()
        .unwrap();

    let repo = api.model("BAAI/bge-small-en-v1.5".to_string());
    let filename = repo.get("config.json").unwrap();

    dbg!(&filename);

    let filename = repo.get("model.safetensors").unwrap();

    dbg!(&filename);

    let filename = repo.get("tokenizer.json").unwrap();

    dbg!(&filename);

    let filename = repo.get("1_Pooling/config.json").unwrap();

    dbg!(&filename);
}
