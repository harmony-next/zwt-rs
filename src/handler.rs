use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};

use crate::image_generator;

// 新增处理单个尺寸的路由处理函数
pub async fn generate_image_with_size(
    Path(size): Path<String>,
) -> impl IntoResponse {
    match image_generator::create_image(&size, None, None, None) {
        Ok(img_data) => (StatusCode::OK, [("Content-Type", "image/png")], img_data),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            [("Content-Type", "text/plain")],
            Vec::from("Invalid parameters"),
        ),
    }
}

pub async fn generate_image(
    Path((size, bg, fg, text)): Path<(String, String, String, String)>,
) -> impl IntoResponse {
    match image_generator::create_image(&size, Some(&bg), Some(&fg), Some(&text)) {
        Ok(img_data) => (StatusCode::OK, [("Content-Type", "image/png")], img_data),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            [("Content-Type", "text/plain")],
            Vec::from("Invalid parameters"),
        ),
    }
}

pub async fn generate_image_without_text(
    Path((size, bg, fg)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match image_generator::create_image(&size, Some(&bg), Some(&fg), None) {
        Ok(img_data) => (StatusCode::OK, [("Content-Type", "image/png")], img_data),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            [("Content-Type", "text/plain")],
            Vec::from("Invalid parameters"),
        ),
    }
} 