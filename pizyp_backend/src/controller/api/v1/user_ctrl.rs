use axum::{http::StatusCode, extract::Json};
use serde::{Deserialize, Serialize};
use crate::db::DB_POOL;
use crate::model::user::User;
use sqlx::Row;
use base64::engine::{general_purpose, Engine as _};
use crate::utils::response::{success, fail, Res};
use chrono::{Local, NaiveDateTime};

#[derive(Debug, Deserialize)]
pub struct MyPageQuery {
    pub uid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BindWalletReq {
    pub pi_uid: String,
    pub wallet_addr: String,
}

#[derive(Debug, Deserialize)]
pub struct BindMiniReq {
    pub pi_uid: String,
    pub mini_openid: String,
}

#[derive(Serialize)]
pub struct UserResp {
    pi_uid: String,
    username: Option<String>,
    nickname: Option<String>,
    upi_num: f64,
    avatar_base64: Option<String>,
    wallet_address: Option<String>,
    bind_mini: bool,
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

// 内部SQL更新登录时间
async fn inner_update_login(pi_uid: &str, now_str: &str) -> Result<(), StatusCode> {
    sqlx::query(
        "UPDATE pi_users SET last_login_at = ?, updated_at = ? WHERE pi_uid = ?",
    )
    .bind(now_str)
    .bind(now_str)
    .bind(pi_uid)
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

// 兼容pi_ctrl旧调用：入参NaiveDateTime，不动pi_ctrl
pub async fn update_user_login_time(pi_uid: &str, now: NaiveDateTime) -> Result<(), StatusCode> {
    let s = now.format("%Y-%m-%d %H:%M:%S").to_string();
    inner_update_login(pi_uid, &s).await
}

// 内部SQL创建用户
async fn inner_create_user(pi_uid: &str, username: &Option<String>, now_str: &str) -> Result<(), StatusCode> {
    sqlx::query(
        "INSERT INTO pi_users (pi_uid, username, nickname, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(pi_uid)
    .bind(username)
    .bind("待映射用户")
    .bind(now_str)
    .bind(now_str)
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

// 兼容pi_ctrl旧调用：入参NaiveDateTime，不动pi_ctrl
pub async fn create_user(pi_uid: &str, username: &Option<String>, now: NaiveDateTime) -> Result<(), StatusCode> {
    let s = now.format("%Y-%m-%d %H:%M:%S").to_string();
    inner_create_user(pi_uid, username, &s).await
}

/// 根据pi_uid查完整用户
pub async fn get_user_by_pi_uid(pi_uid: &str) -> Result<Option<User>, StatusCode> {
    let row = sqlx::query("SELECT * FROM pi_users WHERE pi_uid = ?")
        .bind(pi_uid)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match row {
        Some(r) => Some(User {
            id: r.get("id"),
            pi_uid: r.get("pi_uid"),
            username: r.get("username"),
            invite_code: r.get("invite_code"),
            verify_code: r.get("verify_code"),
            token_claimed: r.get("token_claimed"),
            avatar: r.get("avatar"),
            nickname: r.get("nickname"),
            gender: r.get("gender"),
            bio: r.get("bio"),
            phone: r.get("phone"),
            email: r.get("email"),
            private_key: r.get("private_key"),
            upi_num: r.get("upi_num"),
            status: r.get("status"),
            user_type: r.get("user_type"),
            site_code: r.get("site_code"),
            wallet_address: r.get("wallet_address"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            last_login_at: r.get("last_login_at"),
        }),
        None => None,
    };
    Ok(user)
}

// 个人中心数据接口
pub async fn my_data(Json(params): Json<MyPageQuery>) -> Json<Res<Option<UserResp>>> {
    let pi_uid = match params.uid {
        Some(v) => v,
        None => return fail("用户UID参数缺失"),
    };

    let mut user = match get_user_by_pi_uid(&pi_uid).await {
        Ok(Some(u)) => u,
        Ok(None) => return success(None),
        Err(_) => return fail("查询用户信息失败"),
    };

    // 每次请求实时同步mini最新数据，解决电脑浏览器不同步问题
    if let Some(_pk) = &user.private_key {
        let sync_sql = r#"
            UPDATE pi_users pu
            SET nickname = mu.nickname,
                avatar = mu.avatar_b64_raw,
                upi_num = mu.upi_pending,
                updated_at = CURRENT_TIMESTAMP
            FROM mini_users mu
            WHERE pu.private_key = mu.mini_user_id AND pu.pi_uid = ?
        "#;
        let _ = sqlx::query(sync_sql).bind(&pi_uid).execute(&*DB_POOL).await;
        // 重新拉取刷新后数据
        if let Some(refresh_u) = get_user_by_pi_uid(&pi_uid).await.ok().flatten() {
            user = refresh_u;
        }
    }

    let avatar_base64 = user.avatar.as_ref().map(|b| general_purpose::STANDARD.encode(b));
    let bind_mini = user.private_key.is_some();

    let resp = UserResp {
        pi_uid: user.pi_uid,
        username: user.username,
        nickname: user.nickname,
        upi_num: user.upi_num,
        avatar_base64,
        wallet_address: user.wallet_address,
        bind_mini,
    };

    success(Some(resp))
}

// 绑定钱包接口
pub async fn bind_wallet(Json(body): Json<BindWalletReq>) -> Json<Res<()>> {
    let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let res = sqlx::query(
        "UPDATE pi_users SET wallet_address = ?, updated_at = ? WHERE pi_uid = ?"
    )
    .bind(body.wallet_addr)
    .bind(now_str)
    .bind(body.pi_uid)
    .execute(&*DB_POOL)
    .await;

    match res {
        Ok(_) => success(()),
        Err(e) => {
            eprintln!("bind wallet err {:?}", e);
            fail("钱包绑定失败")
        }
    }
}

// 绑定小程序openid同步
pub async fn bind_mini_account(Json(body): Json<BindMiniReq>) -> Json<Res<()>> {
    let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let pool = &*DB_POOL;

    // 开启事务，保证数据一致性
    let mut tx = pool.begin().await.unwrap();

    // 1. 更新pi_users绑定小程序openid关联
    let update_pi_user = sqlx::query(
        "UPDATE pi_users SET mini_user_id = ?, updated_at = ? WHERE pi_uid = ?"
    )
    .bind(&body.mini_openid)
    .bind(&now_str)
    .bind(&body.pi_uid)
    // 修复：&mut *tx 解包内部连接
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_pi_user {
        eprintln!("update pi_users mini id err {:?}", e);
        let _ = tx.rollback().await;
        return fail("绑定用户标识更新失败");
    }

    // 2. 同步昵称、积分、钱包（仅主表空钱包才回填），头像用户手动设置
    let sync_sql = r#"
        UPDATE pi_users pu
        SET nickname = mu.nickname,
            upi_num = mu.upi_pending,
            wallet_address = CASE WHEN pu.wallet_address IS NULL OR pu.wallet_address = '' THEN mu.wallet ELSE pu.wallet_address END,
            updated_at = ?
        FROM mini_users mu
        WHERE pu.mini_user_id = mu.mini_user_id AND pu.pi_uid = ?
    "#;
    let res = sqlx::query(sync_sql)
        .bind(&now_str)
        .bind(&body.pi_uid)
        // 修复：&mut *tx
        .execute(&mut *tx)
        .await;

    match res {
        Ok(_) => {
            let _ = tx.commit().await;
            success(())
        }
        Err(e) => {
            eprintln!("bind mini sync err {:?}", e);
            let _ = tx.rollback().await;
            fail("小程序账号同步失败，请核对标识")
        }
    }
}