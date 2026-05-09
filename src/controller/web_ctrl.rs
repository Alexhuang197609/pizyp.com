use axum::{
    http::header,
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    Json,
};
use crate::config;
use crate::view;
// 👇【新增】引入action_ctrl控制器
use crate::controller::action_ctrl;
use serde::Serialize;

/// 80端口重定向到HTTPS
pub async fn redirect_to_https() -> impl IntoResponse {
    (
        StatusCode::MOVED_PERMANENTLY,
        [(header::LOCATION, format!("https://{}/", config::DOMAIN))],
    )
}

/// 域名验证密钥接口
pub async fn validation_key() -> impl IntoResponse {
    (
        [("Content-Type", "text/plain")],
        config::VALIDATION_KEY,
    )
}

/// 首页控制器（返回纯HTML静态壳）
pub async fn index() -> impl IntoResponse {
    let html = view::render_index_html().await;
    Html(html)
}

/// 首页动态数据接口（返回JSON，供前端JS渲染）
#[derive(Serialize)]
struct HomeDataResp {
    code: i32,
    list: Vec<crate::model::action::UserAction>,
}
pub async fn home_data() -> impl IntoResponse {
    // ✅ 修复：正确接收 Result 类型
    let list = match action_ctrl::get_user_action_list().await {
        Ok(v) => v,       // 成功：返回数据
        Err(_) => Vec::new(), // 失败：返回空数组，保持兼容
    };

    Json(HomeDataResp {
        code: 0,
        list,
    })
}