use std::time::Duration;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Metadata structure for a token, matching the format expected by Pump.fun.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenMetadata {
    /// Name of the token
    pub name: String,
    /// Token symbol (e.g. "BTC")
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// IPFS URL of the token's image
    pub image: String,
    /// Whether to display the token's name
    pub show_name: bool,
    /// Creation timestamp/source
    pub created_on: String,
    /// Twitter handle
    pub twitter: Option<String>,
    /// Telegram handle
    pub telegram: Option<String>,
    /// Website URL
    pub website: Option<String>,
}

/// Response received after successfully uploading token metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenMetadataIPFS {
    /// The uploaded token metadata
    pub metadata: TokenMetadata,
    /// IPFS URI where the metadata is stored
    pub metadata_uri: String,
}

/// Parameters for creating new token metadata.
#[derive(Debug, Clone)]
pub struct CreateTokenMetadata {
    /// Name of the token
    pub name: String,
    /// Token symbol (e.g. "BTC")
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// Path to the token's image file
    pub file: String,
    /// Optional Twitter handle
    pub twitter: Option<String>,
    /// Optional Telegram group
    pub telegram: Option<String>,
    /// Optional website URL
    pub website: Option<String>,

    pub metadata_uri: Option<String>,
}

pub async fn create_token_metadata(
    metadata: CreateTokenMetadata,
    jwt_token: &str,
) -> Result<TokenMetadataIPFS, anyhow::Error> {
    let ipfs_url: String = if metadata.file.starts_with("http") || metadata.metadata_uri.is_some() {
        metadata.file
    } else {
        let base64_string = file_to_base64(&metadata.file).await?;
        upload_base64_file(&base64_string, jwt_token).await?
    };

    // Private code
}

pub async fn upload_base64_file(
    base64_string: &str,
    jwt_token: &str,
) -> Result<String, anyhow::Error> {
    let _decoded_bytes = general_purpose::STANDARD.decode(base64_string)?;

    let client = Client::builder()
        .timeout(Duration::from_secs(120)) 
        .pool_max_idle_per_host(0) 
        .pool_idle_timeout(None) 
        .build()?;

    // let part = Part::bytes(decoded_bytes)
    //     .file_name("file.png") 
    //     .mime_str("image/png")?; 

    // let form = Form::new().part("file", part);

    // Private code
}

async fn file_to_base64(file_path: &str) -> Result<String, anyhow::Error> {
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    Ok(base64_string)
}
