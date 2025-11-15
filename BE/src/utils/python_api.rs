use reqwest::multipart;
use std::env;
use crate::models::python_api_response::PythonApiResponse;

/// Gọi Python API để extract embedding từ ảnh
pub async fn extract_embedding_from_image(image_bytes: Vec<u8>, file_name: &str) -> Result<Vec<f32>, String> {
    let python_api_url = env::var("PYTHON_API_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    let url = format!("{}/api/ai/extract-embedding", python_api_url);

    let part = multipart::Part::bytes(image_bytes)
        .file_name(file_name.to_string())
        .mime_str("image/jpeg")
        .map_err(|e| format!("Failed to create multipart part: {}", e))?;

    let form = multipart::Form::new().part("file", part);

    let client = reqwest::Client::new();

    let res = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Request to Python API failed: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Python API error ({}): {}", status, text));
    }

    let body: PythonApiResponse = res.json().await
        .map_err(|e| format!("Failed to parse Python API response: {}", e))?;

    if body.status != "success" {
        return Err("Python API returned non-success status".to_string());
    }

    Ok(body.embedding)
}

