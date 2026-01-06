use std::{
    env::{self},
    fs,
    os::unix::fs::symlink,
    path::Path,
};

use anyhow::Context;
use hf_hub::api::tokio::Api;

/// Download model files from Huggingface if needed
/// If the current version exists already nothing will be downloaded
///
/// May require token in ~/.cache/huggingface/token
///
/// # Errors
/// - when we fail to create the `hf_hub` API client
/// - when we fail to download any of the model files
/// - when we fail to get $HOME environment variable
pub async fn download_model() -> anyhow::Result<()> {
    let model_name = env::var("WIS_RS_STT_MODEL").unwrap_or(String::from("model.onnx"));

    let model = format!("onnx/{model_name}");
    let model_data = format!("onnx/{model_name}_data");

    let api = Api::new()?;

    let model_path = api
        .model(String::from("onnx-community/parakeet-ctc-0.6b-ONNX"))
        .get(&model)
        .await?;

    tracing::info!("{model_path:?}");

    let model_data_path = api
        .model(String::from("onnx-community/parakeet-ctc-0.6b-ONNX"))
        .get(&model_data)
        .await?;

    tracing::info!("{model_data_path:?}");

    let model_dir = model_data_path
        .parent()
        .context("model directory has no parent")?;

    let tokenizer_path = api
        .model(String::from("onnx-community/parakeet-ctc-0.6b-ONNX"))
        .get("tokenizer.json")
        .await?;

    let tokenizer_link = model_dir.join("tokenizer.json");

    match fs::symlink_metadata(&tokenizer_link) {
        Err(e) => {
            tracing::error!("tokenizer_link: {e}");
            symlink(tokenizer_path, Path::new(&tokenizer_link))?;
        }
        Ok(tokenizer_link_stat) => {
            if !tokenizer_link_stat.is_symlink() {
                tracing::info!("tokenizer_link: {tokenizer_link:?}");
                tracing::info!("tokenizer_link_stat: {tokenizer_link_stat:?}");
                symlink(tokenizer_path, Path::new(&tokenizer_link))?;
            }
        }
    }

    Ok(())
}
