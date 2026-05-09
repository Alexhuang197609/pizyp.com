use axum::{http::StatusCode, response::IntoResponse};
use std::fs::read;



// 给 banner 也用完全一样的写法，只返回二元组
pub async fn banner1() -> impl IntoResponse {
    match read("./static/banner1.jpg") {
        Ok(data) => (StatusCode::OK, data),
        Err(_) => (StatusCode::NOT_FOUND, b"Not Found".to_vec()),
    }
}

pub async fn banner2() -> impl IntoResponse {
    match read("./static/banner2.jpg") {
        Ok(data) => (StatusCode::OK, data),
        Err(_) => (StatusCode::NOT_FOUND, b"Not Found".to_vec()),
    }
}

pub async fn banner3() -> impl IntoResponse {
    match read("./static/banner3.jpg") {
        Ok(data) => (StatusCode::OK, data),
        Err(_) => (StatusCode::NOT_FOUND, b"Not Found".to_vec()),
    }
}

pub async fn qrcode() -> impl IntoResponse {
    match read("./static/qrcode.jpg") {
        Ok(data) => (StatusCode::OK, data),
        Err(_) => (StatusCode::NOT_FOUND, b"Not Found".to_vec()),
    }
}
