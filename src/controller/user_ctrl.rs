use axum::{http::StatusCode, response::Html, extract::Query, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::db::DB_POOL;
use crate::model::user::User;
use crate::view::my::render_my_page;
use chrono::NaiveDateTime;
use sqlx::Row;
use base64::engine::{general_purpose, Engine as _};

// 定义接收 ?uid=xxx 的查询参数结构体
#[derive(Debug, Deserialize)]
pub struct MyPageQuery {
    pub uid: Option<String>,
}

/// 根据 pi_uid 查询用户是否存在
pub async fn check_user_exists(pi_uid: &str) -> Result<bool, StatusCode> {
    let exists = sqlx::query("SELECT 1 FROM pi_users WHERE pi_uid = ?")
        .bind(pi_uid)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some();

    Ok(exists)
}

/// 新建用户（注册）
pub async fn create_user(
    pi_uid: &str,
    username: &Option<String>,
    now: NaiveDateTime,
) -> Result<(), StatusCode> {
    sqlx::query(
        "INSERT INTO pi_users (pi_uid, username, nickname, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(pi_uid)
    .bind(username)
    .bind("待映射用户") 
    .bind(now)
    .bind(now)
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// 更新用户最后登录时间
pub async fn update_user_login_time(
    pi_uid: &str,
    now: NaiveDateTime,
) -> Result<(), StatusCode> {
    sqlx::query(
        "UPDATE pi_users 
         SET last_login_at = ?, updated_at = ? 
         WHERE pi_uid = ?",
    )
    .bind(now)
    .bind(now)
    .bind(pi_uid)
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// 根据 pi_uid 获取单条完整用户信息
pub async fn get_user_by_pi_uid(pi_uid: &str) -> Result<Option<User>, StatusCode> {
    let row = sqlx::query("SELECT * FROM pi_users WHERE pi_uid = ?")
        .bind(pi_uid)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match row {
        Some(row) => Some(User {
            id: row.get("id"),
            pi_uid: row.get("pi_uid"),
            username: row.get("username"),
            invite_code: row.get("invite_code"),
            verify_code: row.get("verify_code"),
            token_claimed: row.get("token_claimed"),
            avatar: row.get("avatar"),
            nickname: row.get("nickname"),
            gender: row.get("gender"),
            bio: row.get("bio"),
            phone: row.get("phone"),
            email: row.get("email"),
            private_key: row.get("private_key"),
            upi_num: row.get("upi_num"),
            status: row.get("status"),
            user_type: row.get("user_type"),
            site_code: row.get("site_code"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login_at: row.get("last_login_at"),
        }),
        None => None,
    };

    Ok(user)
}

// 个人中心页面入口（改成返回纯壳）
pub async fn my_page(Query(_params): Query<MyPageQuery>) -> Html<String> {
    let html = render_my_page().await;
    Html(html)
}

// ===================== 新增：我的页面异步接口 =====================
#[derive(Serialize)]
pub struct MyDataResp {
    code: i32,
    user: Option<UserResp>,
}

#[derive(Serialize)]
pub struct UserResp {
    username: Option<String>,
    nickname: Option<String>,
    upi_num: f64,  // 👈 修复：改成 f64
    avatar_base64: Option<String>,
}

pub async fn my_data(Query(params): Query<MyPageQuery>) -> impl IntoResponse {
    let pi_uid = match params.uid {
        Some(uid) => uid,
        None => return Json(MyDataResp { code: -1, user: None }),
    };

    let user = match get_user_by_pi_uid(&pi_uid).await {
        Ok(Some(u)) => u,
        _ => return Json(MyDataResp { code: -1, user: None }),
    };

    let avatar_base64 = user.avatar.as_ref().map(|b| general_purpose::STANDARD.encode(b));

    Json(MyDataResp {
        code: 0,
        user: Some(UserResp {
            username: user.username,
            nickname: user.nickname,
            upi_num: user.upi_num, // 👈 现在类型匹配
            avatar_base64,
        }),
    })
}