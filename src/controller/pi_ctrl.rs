//src/controller/pi_ctrl.rs
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    extract::Path,
    Json,
};
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use crate::config::{PI_API_KEY, PI_API_BASE};
use crate::controller::order_ctrl::update_order_to_paid;
use crate::db::DB_POOL;
use chrono::prelude::*;
// 引入 user_ctrl 数据库操作
use crate::controller::user_ctrl::{check_user_exists, create_user, update_user_login_time};

#[derive(Debug, Deserialize)]
struct PiMeResponse {
    pub uid: String,
    pub username: Option<String>,
    pub credentials: Credentials,
}

#[derive(Debug, Deserialize)]
struct Credentials {
    pub scopes: Vec<String>,
}

// ====================== 核心：Pi 鉴权（无数据库代码） ======================
pub async fn pi_verify(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    println!("收到授权请求，开始处理...");

    let auth_header = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let client = Client::new();
    let res = client
        .get("https://api.minepi.com/v2/me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !res.status().is_success() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_info: PiMeResponse = res
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("\n===== Pi 用户授权成功 =====");
    println!("UID: {}", user_info.uid);
    if let Some(uname) = &user_info.username {
        println!("用户名: {}", uname);
    }
    println!("============================\n");

    let now = chrono::Utc::now().naive_utc();

    // ===== 全部调用 user_ctrl，pi_ctrl 不再操作数据库 =====
    let exists = check_user_exists(&user_info.uid).await?;

    if exists {
        update_user_login_time(&user_info.uid, now).await?;
        println!("用户已存在，更新登录时间");
    } else {
        create_user(&user_info.uid, &user_info.username, now).await?;
        println!("新用户，创建入库");
    }

    Ok(StatusCode::OK)
}

// --------------------------
// Pi 支付相关 DTO
// --------------------------
#[derive(Debug, Deserialize)]
pub struct PaymentApproveReq {
    pub order_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentCompleteReq {
    pub order_id: String,
    pub txid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiPaymentDTO {
    pub payment_id: String,
    pub amount: f64,
    pub memo: String,
    pub status: String,
    pub txid: Option<String>,
}

// 插入支付记录（仅商城订单）
pub async fn insert_payment_record(
    order_id: i64,
    pi_uid: &str,
    payment_id: &str,
    amount: f64,
) -> Result<(), StatusCode> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query!(
        "INSERT INTO shop_payment (order_id, pi_uid, payment_id, amount, status, create_time, update_time)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        order_id,
        pi_uid,
        payment_id,
        amount,
        1, // 已批准
        now,
        now
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

// 更新支付记录为已完成（仅商城订单）
pub async fn update_payment_completed(
    payment_id: &str,
    txid: &str
) -> Result<(), StatusCode> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query!(
        "UPDATE shop_payment SET status = 2, txid = ?, update_time = ? WHERE payment_id = ?",
        txid,
        now,
        payment_id
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

// --------------------------
// 支付审批接口
// --------------------------
pub async fn pi_payment_approve(
    Path(payment_id): Path<String>,
    Json(_req): Json<PaymentApproveReq>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("\n===== 收到支付审批请求 =====");
    println!("payment_id: {}", payment_id);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = format!("{}/payments/{}/approve", PI_API_BASE, payment_id);
    println!("请求Pi官方接口: {}", url);

    let res = match client
        .post(&url)
        .header("Authorization", format!("Key {}", PI_API_KEY))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ 网络请求Pi接口失败: {}", e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        eprintln!("❌ Pi接口返回错误: {}", err_text);
        return Err(StatusCode::BAD_GATEWAY);
    }

    let data: serde_json::Value = res
        .json()
        .await
        .map_err(|e| {
            eprintln!("❌ 解析Pi返回数据失败: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    println!("✅ Pi 接口返回数据: {:?}", data);

    // 仅商城订单支付才写入 shop_payment
    if let Some(metadata) = data.get("metadata").and_then(|m| m.as_object()) {
        if metadata.get("type") == Some(&serde_json::Value::String("order".into())) {
            let order_id = _req.order_id.parse::<i64>().unwrap_or(0);
            let amount = data.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let pi_uid = data.get("user").and_then(|u| u.get("uid")).and_then(|v| v.as_str()).unwrap_or("");
            
            let _ = insert_payment_record(order_id, pi_uid, &payment_id, amount).await;
        }
    }

    Ok(Json(serde_json::json!({
        "status": "approved",
        "payment_id": payment_id
    })))
}

// --------------------------
// 支付完成接口
// --------------------------
pub async fn pi_payment_complete(
    Path(payment_id): Path<String>,
    Json(req): Json<PaymentCompleteReq>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("\n===== 收到支付完成请求 =====");
    println!("payment_id: {}, txid: {}", payment_id, req.txid);

    let url = format!("{}/payments/{}/complete", PI_API_BASE, payment_id);
    println!("请求Pi官方接口: {}", url);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res = client
        .post(&url)
        .header("Authorization", format!("Key {}", PI_API_KEY))
        .json(&serde_json::json!({ "txid": req.txid }))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ 网络请求Pi接口失败: {}", e);
            StatusCode::BAD_GATEWAY
        })?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        eprintln!("❌ Pi接口返回错误: {}", err_text);
        return Err(StatusCode::BAD_GATEWAY);
    }

    let data: serde_json::Value = res
        .json()
        .await
        .map_err(|e| {
            eprintln!("❌ 解析Pi返回数据失败: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    println!("✅ Pi complete 接口返回数据: {:?}", data);

    // 仅商城订单支付才更新订单+支付记录
    if let Some(metadata) = data.get("metadata").and_then(|m| m.as_object()) {
        if metadata.get("type") == Some(&serde_json::Value::String("order".into())) {
            let order_id = req.order_id.parse::<i64>().unwrap_or(0);
            
            let _ = update_order_to_paid(order_id).await;
            let _ = update_payment_completed(&payment_id, &req.txid).await;
        }
    }

    Ok(Json(serde_json::json!({
        "status": "completed",
        "payment_id": payment_id,
        "txid": req.txid
    })))
}