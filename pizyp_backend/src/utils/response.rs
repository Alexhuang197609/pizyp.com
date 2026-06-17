use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

// 成功：直接返回 Json<Res<T>>
pub fn success<T>(data: T) -> Json<Res<T>> {
    Json(Res {
        code: 0,
        msg: "ok".to_string(),
        data: Some(data),
    })
}

// 失败：直接返回 Json<Res<T>>
pub fn fail<T>(msg: &str) -> Json<Res<T>> {
    Json(Res {
        code: -1,
        msg: msg.to_string(),
        data: None,
    })
}