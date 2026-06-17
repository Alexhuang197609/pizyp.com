use axum::{
    http::{HeaderMap, StatusCode},
    Json, extract::Path,
};
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use crate::config::{PI_API_KEY, PI_API_BASE};
use crate::controller::api::v1::order_ctrl::update_order_to_paid;
use crate::controller::api::v1::user_ctrl::{check_user_exists, create_user, update_user_login_time};
use crate::utils::response::{success, fail, Res};
use chrono::prelude::*;
use crate::db::DB_POOL;

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

// Pi 鉴权
pub async fn pi_verify(headers: HeaderMap) -> Json<Res<()>> {
    println!("收到授权请求，开始处理...");

    let auth_header = match headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => return fail("未携带有效授权Token"),
    };

    let client = Client::new();
    let res = match client
        .get("https://api.minepi.com/v2/me")
        .header("Authorization", format!("Bearer {}", auth_header))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Pi鉴权网络错误: {:?}", e);
            return fail("Pi服务网络请求失败");
        }
    };

    if !res.status().is_success() {
        return fail("Pi身份验证失败");
    }

    let user_info: PiMeResponse = match res.json().await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Pi用户信息解析失败: {:?}", e);
            return fail("解析用户信息异常");
        }
    };

    println!("\n===== Pi 用户授权成功 =====");
    println!("UID: {}", user_info.uid);
    if let Some(uname) = &user_info.username {
        println!("用户名: {}", uname);
    }
    println!("============================\n");

    let now = chrono::Utc::now().naive_utc();
    let exists = match check_user_exists(&user_info.uid).await {
        Ok(b) => b,
        Err(_) => return fail("校验用户库失败"),
    };

    if exists {
        if update_user_login_time(&user_info.uid, now).await.is_ok() {
            println!("用户已存在，更新登录时间");
        }
    } else {
        if create_user(&user_info.uid, &user_info.username, now).await.is_ok() {
            println!("新用户，创建入库");
        }
    }

    success(())
}

// 支付请求结构体
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

/// 插入支付记录
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
        1,
        now,
        now
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// 更新支付记录为已完成
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

/// 支付审批接口
pub async fn pi_payment_approve(
    Path(payment_id): Path<String>,
    Json(_req): Json<PaymentApproveReq>,
) -> Json<Res<serde_json::Value>> {
    println!("\n===== 收到支付审批请求 =====");
    println!("payment_id: {}", payment_id);

    let client = match Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("创建http客户端失败: {:?}", e);
            return fail("服务初始化异常");
        }
    };

    let url = format!("{}/payments/{}/approve", PI_API_BASE, payment_id);
    println!("请求Pi官方接口: {}", url);

    // 分步获取支付信息，不再使用and_then包裹异步json
    let get_url = format!("{}/payments/{}", PI_API_BASE, payment_id);
    let get_resp_res = client
        .get(&get_url)
        .header("Authorization", format!("Key {}", PI_API_KEY))
        .send()
        .await;

    let get_resp = match get_resp_res {
        Ok(r) => r,
        Err(e) => {
            eprintln!("拉取支付信息失败: {:?}", e);
            return fail("获取Pi支付信息失败");
        }
    };
    let payment_data = match get_resp.json::<serde_json::Value>().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("解析支付JSON失败: {:?}", e);
            return fail("解析支付返回数据异常");
        }
    };

    // 发起审批
    let res = match client
        .post(&url)
        .header("Authorization", format!("Key {}", PI_API_KEY))
        .send()
        .await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ 网络请求Pi接口失败: {}", e);
            return fail("Pi网关请求异常");
        }
    };

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        eprintln!("❌ Pi接口返回错误: {}", err_text);
        return fail("Pi审批接口调用失败");
    }

    let data = match res.json::<serde_json::Value>().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ 解析Pi返回数据失败: {}", e);
            return fail("解析支付返回数据异常");
        }
    };
    println!("✅ Pi 接口返回数据: {:?}", data);

    if let Some(metadata) = data.get("metadata").and_then(|m| m.as_object()) {
        if metadata.get("type") == Some(&serde_json::Value::String("order".into())) {
            let order_id = _req.order_id.parse::<i64>().unwrap_or(0);
            let amount = data.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let pi_uid = data.get("user").and_then(|u| u.get("uid")).and_then(|v| v.as_str()).unwrap_or("");
            let _ = insert_payment_record(order_id, pi_uid, &payment_id, amount).await;
        }
    }

    success(serde_json::json!({
        "status": "approved",
        "payment_id": payment_id
    }))
}

/// 支付完成回调接口
pub async fn pi_payment_complete(
    Path(payment_id): Path<String>,
    Json(req): Json<PaymentCompleteReq>,
) -> Json<Res<serde_json::Value>> {
    println!("\n===== 收到支付完成请求 =====");
    println!("payment_id: {}, txid: {}", payment_id, req.txid);

    let url = format!("{}/payments/{}/complete", PI_API_BASE, payment_id);
    println!("请求Pi官方接口: {}", url);

    let client = match Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("创建http客户端失败: {:?}", e);
            return fail("服务初始化异常");
        }
    };

    let res = match client
        .post(&url)
        .header("Authorization", format!("Key {}", PI_API_KEY))
        .json(&serde_json::json!({ "txid": req.txid }))
        .send()
        .await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ 网络请求Pi接口失败: {}", e);
            return fail("Pi网关请求异常");
        }
    };

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        eprintln!("❌ Pi接口返回错误: {}", err_text);
        return fail("Pi完成接口调用失败");
    }

    let data = match res.json::<serde_json::Value>().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ 解析Pi返回数据失败: {}", e);
            return fail("解析支付返回数据异常");
        }
    };
    println!("✅ Pi complete 接口返回数据: {:?}", data);

    if let Some(metadata) = data.get("metadata").and_then(|m| m.as_object()) {
        if metadata.get("type") == Some(&serde_json::Value::String("order".into())) {
            let order_id = req.order_id.parse::<i64>().unwrap_or(0);
            let _ = update_order_to_paid(order_id).await;
            let _ = update_payment_completed(&payment_id, &req.txid).await;
        }
    }

    success(serde_json::json!({
        "status": "completed",
        "payment_id": payment_id,
        "txid": req.txid
    }))
}