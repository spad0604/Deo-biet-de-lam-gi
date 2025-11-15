use serde::Deserialize;

#[derive(Deserialize)]
pub struct PythonApiResponse {
    pub status: String,
    pub embedding: Vec<f32>,
}