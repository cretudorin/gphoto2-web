use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: i32,
    pub message: String,
}
