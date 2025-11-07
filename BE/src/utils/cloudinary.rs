use reqwest::multipart;
use serde::Deserialize;
use std::env;
use std::fs::FileType;

#[derive(Deserialize)]
pub struct CloudinaryResponse {
    pub secure_url: String
}

pub async fn upload_to_cloudinary(file_bytes:Vec<u8>, file_name: &str) -> Result<String, String> {
    let cloud_name = env::var("CLOUDINARY_CLOUD_NAME").map_err(|_| "Missing CLOUDINARY_CLOUD_NAME")?;
    let api_key = env::var("CLOUDINARY_API_KEY").map_err(|_| "Missing CLOUDINARY_API_KEY")?;
    let api_secret = env::var("CLOUDINARY_API_SECRET").map_err(|_| "Missing CLOUDINARY_API_SECRET")?;

    let url = format!("https://api.cloudinary.com/v1_1/{}/image/upload", cloud_name);

    let part = multipart::Part::bytes(file_bytes)
        .file_name(file_name.to_string())
        .mime_str("image/jpeg")
        .unwrap();

    let form = multipart::Form::new()
        .part("file", part)
        .text("upload_preset", "unsigned_preset");

    let client = reqwest::Client::new();

    let res = client
        .post(&url)
        .basic_auth(api_key, Some(api_secret))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        let text = res.text().await.map_err(|e| format!("Failed to read response: {}", e))?;
        return Err(text);
    }

    let body : CloudinaryResponse = res.json().await.map_err(|e| format!("Failed to read response: {}", e))?;
    Ok(body.secure_url)
}