use axum::{http::StatusCode, Json, extract::Json as ExtractJson};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::DB_POOL;
use chrono::prelude::*;
use crate::utils::response::{success, fail, Res};

// 订单状态常量
pub const ORDER_STATUS_PENDING: i64 = 0; // 待支付
pub const ORDER_STATUS_PAID: i64 = 1;    // 已支付
pub const ORDER_STATUS_FINISHED: i64 = 2; // 已完成

/// 立即购买（单件商品）
#[derive(Debug, Deserialize)]
pub struct OrderBuyNowParams {
    pub uid: String,
    pub goods_id: i64,
    pub goods_spec: Option<String>,
    pub num: Option<i32>,
}

/// 购物车去结算（多商品）
#[derive(Debug, Deserialize)]
pub struct OrderSettleParams {
    pub uid: String,
    pub cart_ids: Vec<i64>,
}

/// 生成唯一订单号
pub fn generate_order_no() -> String {
    let now = Local::now();
    format!("ORD{}", now.format("%Y%m%d%H%M%S%f"))
}

/// 创建订单主表
pub async fn insert_order(
    uid: &str,
    order_no: &str,
    total_price_pi: f64,
) -> Result<i64, StatusCode> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let id = sqlx::query!(
        "INSERT INTO shop_order (uid, order_no, total_price_pi, status, create_time, update_time)
         VALUES (?, ?, ?, ?, ?, ?)",
        uid,
        order_no,
        total_price_pi,
        ORDER_STATUS_PENDING,
        now,
        now
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .last_insert_rowid();

    Ok(id)
}

/// 创建订单明细表
pub async fn insert_order_item(
    order_id: i64,
    goods_id: i64,
    goods_name: &str,
    goods_spec: &str,
    price_pi: f64,
    num: i32,
) -> Result<(), StatusCode> {
    sqlx::query!(
        "INSERT INTO shop_order_item (order_id, goods_id, goods_name, goods_spec, price_pi, num)
         VALUES (?, ?, ?, ?, ?, ?)",
        order_id,
        goods_id,
        goods_name,
        goods_spec,
        price_pi,
        num
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// 根据购物车ID批量查询购物车数据
pub async fn get_cart_list_by_ids(
    uid: &str,
    cart_ids: &[i64],
) -> Result<Vec<sqlx::sqlite::SqliteRow>, StatusCode> {
    let placeholders = vec!["?"; cart_ids.len()].join(",");
    let sql = format!(
        "SELECT id, goods_id, goods_name, goods_spec, price_pi, num
         FROM cart
         WHERE uid = ? AND id IN ({})",
        placeholders
    );

    let mut query = sqlx::query(&sql).bind(uid);
    for &id in cart_ids {
        query = query.bind(id);
    }

    let rows = query
        .fetch_all(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(rows)
}

/// 删除已结算的购物车
pub async fn delete_cart_by_ids(cart_ids: &[i64]) -> Result<(), StatusCode> {
    let placeholders = vec!["?"; cart_ids.len()].join(",");
    let sql = format!("DELETE FROM cart WHERE id IN ({})", placeholders);

    let mut query = sqlx::query(&sql);
    for &id in cart_ids {
        query = query.bind(id);
    }

    query
        .execute(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// 删除待支付订单
pub async fn delete_pending_order(
    uid: &str,
    order_id: i64,
) -> Result<bool, axum::http::StatusCode> {
    let _ = sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&*DB_POOL)
        .await;

    let _ = sqlx::query("DELETE FROM shop_order_item WHERE order_id = ?")
        .bind(order_id)
        .execute(&*DB_POOL)
        .await;

    let result = sqlx::query("DELETE FROM shop_order WHERE id = ?")
        .bind(order_id)
        .execute(&*DB_POOL)
        .await;

    let _ = sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&*DB_POOL)
        .await;

    match result {
        Ok(r) => Ok(r.rows_affected() > 0),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 更新订单为已支付
pub async fn update_order_to_paid(order_id: i64) -> Result<(), StatusCode> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query!(
        "UPDATE shop_order SET status = 1, update_time = ? WHERE id = ?",
        now,
        order_id
    )
    .execute(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

// ---------------------- 对外接口Handler ----------------------
pub async fn order_buy_now(
    ExtractJson(params): ExtractJson<OrderBuyNowParams>,
) -> Json<Res<serde_json::Value>> {
    let uid = params.uid.trim();
    let goods_id = params.goods_id;
    let goods_spec = params.goods_spec.unwrap_or_default();
    let num = params.num.unwrap_or(1);

    if uid.is_empty() || goods_id <= 0 || num <= 0 {
        return fail("参数非法");
    }

    let goods = match sqlx::query!(
        "SELECT goods_name, price_pi FROM shop_goods WHERE id = ?",
        goods_id
    )
    .fetch_one(&*DB_POOL)
    .await {
        Ok(v) => v,
        Err(_) => return fail("商品不存在"),
    };

    let total_price = goods.price_pi * num as f64;
    let order_no = generate_order_no();
    let order_id = match insert_order(uid, &order_no, total_price).await {
        Ok(v) => v,
        Err(_) => return fail("创建订单失败"),
    };

    if insert_order_item(order_id, goods_id, &goods.goods_name, &goods_spec, goods.price_pi, num).await.is_err() {
        return fail("写入订单明细失败");
    }

    success(serde_json::json!({
        "order_no": order_no,
        "total_price": total_price
    }))
}

pub async fn order_settle(
    ExtractJson(params): ExtractJson<OrderSettleParams>,
) -> Json<Res<serde_json::Value>> {
    let uid = params.uid.trim();
    let cart_ids = params.cart_ids;

    if uid.is_empty() || cart_ids.is_empty() {
        return fail("参数非法");
    }

    let rows = match get_cart_list_by_ids(uid, &cart_ids).await {
        Ok(v) => v,
        Err(_) => return fail("读取购物车失败"),
    };
    if rows.is_empty() {
        return fail("购物车数据不存在");
    }

    let mut total_price = 0.0;
    for row in &rows {
        let price: f64 = row.get("price_pi");
        let num: i32 = row.get("num");
        total_price += price * num as f64;
    }

    let order_no = generate_order_no();
    let order_id = match insert_order(uid, &order_no, total_price).await {
        Ok(v) => v,
        Err(_) => return fail("创建订单失败"),
    };

    for row in rows {
        let goods_id: i64 = row.get("goods_id");
        let goods_name: String = row.get("goods_name");
        let goods_spec: String = row.get("goods_spec");
        let price_pi: f64 = row.get("price_pi");
        let num: i32 = row.get("num");
        let _ = insert_order_item(order_id, goods_id, &goods_name, &goods_spec, price_pi, num).await;
    }

    let _ = delete_cart_by_ids(&cart_ids).await;
    success(serde_json::json!({
        "order_no": order_no,
        "total_price": total_price
    }))
}

pub async fn order_delete(
    axum::extract::Json(params): axum::extract::Json<serde_json::Value>,
) -> Json<Res<()>> {
    let uid = params["uid"].as_str().unwrap_or_default();
    let order_id_str = params["order_id"].as_str().unwrap_or_default();
    let order_id: i64 = order_id_str.parse().unwrap_or_default();

    match delete_pending_order(uid, order_id).await {
        Ok(true) => success(()),
        Ok(false) => fail("删除失败：订单不存在或非待支付"),
        Err(_) => fail("服务器异常"),
    }
}

pub async fn get_order_list(
    ExtractJson(params): ExtractJson<serde_json::Value>,
) -> Json<Res<Vec<serde_json::Value>>> {
    let uid = match params["uid"].as_str() {
        Some(v) => v.trim(),
        None => return success(Vec::new()),
    };
    if uid.is_empty() {
        return success(Vec::new());
    }

    let orders = match sqlx::query!(
        "SELECT id, order_no, total_price_pi, status, create_time FROM shop_order WHERE uid = ? ORDER BY create_time DESC",
        uid
    )
    .fetch_all(&*DB_POOL)
    .await {
        Ok(v) => v,
        Err(_) => return fail("查询订单失败"),
    };

    let mut list = Vec::new();
    for o in orders {
        let status_text = match o.status {
            ORDER_STATUS_PENDING => "待支付",
            ORDER_STATUS_PAID => "已支付",
            ORDER_STATUS_FINISHED => "已完成",
            _ => "未知状态",
        }.to_string();

        list.push(serde_json::json!({
            "id": o.id,
            "order_no": o.order_no,
            "total_price_pi": o.total_price_pi,
            "status": o.status,
            "status_text": status_text,
            "create_time": o.create_time
        }));
    }
    success(list)
}

pub async fn order_detail(
    axum::extract::Json(params): axum::extract::Json<serde_json::Value>,
) -> Json<Res<serde_json::Value>> {
    let uid = params["uid"].as_str().unwrap_or_default();
    let order_id_str = params["order_id"].as_str().unwrap_or_default();
    let order_id: i64 = order_id_str.parse().unwrap_or_default();

    let order = match sqlx::query!(
        r#"SELECT id,order_no,total_price_pi,status,create_time FROM shop_order WHERE id = ? AND uid = ?"#,
        order_id,
        uid
    )
    .fetch_optional(&*DB_POOL)
    .await {
        Ok(Some(o)) => o,
        _ => return fail("订单不存在"),
    };

    let items = sqlx::query!(
        r#"SELECT goods_name,goods_spec,price_pi,num FROM shop_order_item WHERE order_id = ?"#,
        order_id
    )
    .fetch_all(&*DB_POOL)
    .await
    .unwrap_or_default();

    let mut list = Vec::new();
    for item in items {
        list.push(serde_json::json!({
            "goods_name": item.goods_name,
            "goods_spec": item.goods_spec,
            "price_pi": item.price_pi,
            "num": item.num
        }));
    }

    let status_text = match order.status {
        0 => "待支付",
        1 => "已支付",
        2 => "已完成",
        _ => "未知状态",
    };

    let resp_data = serde_json::json!({
        "order": {
            "order_no": order.order_no,
            "total_price_pi": order.total_price_pi,
            "status": order.status,
            "status_text": status_text,
            "create_time": order.create_time
        },
        "list": list
    });
    success(resp_data)
}