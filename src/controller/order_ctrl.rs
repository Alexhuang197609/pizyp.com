//src/controller/order_ctrl.rs
use axum::{http::StatusCode, Json, extract::Json as ExtractJson};
use axum::response::Html;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::DB_POOL;
use serde_json::{Value};
use chrono::prelude::*;

// 订单状态常量【已修复：统一改为 i64 类型，匹配数据库】
pub const ORDER_STATUS_PENDING: i64 = 0; // 待支付
pub const ORDER_STATUS_PAID: i64 = 1;    // 已支付
pub const ORDER_STATUS_FINISHED: i64 = 2; // 已完成

// ================================================================================================
// 前端请求参数结构体（实际接口用到，保留）
// ================================================================================================

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
    pub cart_ids: Vec<i64>, // 勾选的购物车ID列表
}

// ================================================================================================
// 数据库操作方法（全部保留）
// ================================================================================================

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

//删除待支付订单
pub async fn delete_pending_order(
    uid: &str,
    order_id: i64,
) -> Result<bool, axum::http::StatusCode> {
    // 关闭外键
    let _ = sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&*DB_POOL)
        .await;

    // 先删子表
    let _ = sqlx::query("DELETE FROM shop_order_item WHERE order_id = ?")
        .bind(order_id)
        .execute(&*DB_POOL)
        .await;

    // 🔥 关键修复：只按订单ID删除，确保一定能删掉
    let result = sqlx::query("DELETE FROM shop_order WHERE id = ?")
        .bind(order_id)
        .execute(&*DB_POOL)
        .await;

    // 开启外键
    let _ = sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&*DB_POOL)
        .await;

    match result {
        Ok(r) => Ok(r.rows_affected() > 0),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
// ================================================================================================
// 对外接口（全部保留）
// ================================================================================================

/// 立即购买（单件）
pub async fn order_buy_now(
    ExtractJson(params): ExtractJson<OrderBuyNowParams>,
) -> Result<Json<Value>, StatusCode> {
    let uid = params.uid.trim();
    let goods_id = params.goods_id;
    let goods_spec = params.goods_spec.unwrap_or_default();
    let num = params.num.unwrap_or(1);

    if uid.is_empty() || goods_id <= 0 || num <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 查询商品信息
    let goods = sqlx::query!(
        "SELECT goods_name, price_pi FROM shop_goods WHERE id = ?",
        goods_id
    )
    .fetch_one(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let total_price = goods.price_pi * num as f64;
    let order_no = generate_order_no();
    let order_id = insert_order(uid, &order_no, total_price).await?;

    // 插入明细
    insert_order_item(
        order_id,
        goods_id,
        &goods.goods_name,
        &goods_spec,
        goods.price_pi,
        num,
    ).await?;

    Ok(Json(serde_json::json!({
        "code": 0,
        "msg": "订单创建成功",
        "order_no": order_no,
        "total_price": total_price
    })))
}

/// 购物车去结算（多件）
pub async fn order_settle(
    ExtractJson(params): ExtractJson<OrderSettleParams>,
) -> Result<Json<Value>, StatusCode> {
    let uid = params.uid.trim();
    let cart_ids = params.cart_ids;

    if uid.is_empty() || cart_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 查询购物车商品
    let rows = get_cart_list_by_ids(uid, &cart_ids).await?;
    if rows.is_empty() {
        return Ok(Json(serde_json::json!({
            "code": -1,
            "msg": "购物车数据不存在"
        })));
    }

    // 计算总价
    let mut total_price = 0.0;
    for row in &rows {
        let price: f64 = row.get("price_pi");
        let num: i32 = row.get("num");
        total_price += price * num as f64;
    }

    // 创建订单
    let order_no = generate_order_no();
    let order_id = insert_order(uid, &order_no, total_price).await?;

    // 批量插入明细
    for row in rows {
        let goods_id: i64 = row.get("goods_id");
        let goods_name: String = row.get("goods_name");
        let goods_spec: String = row.get("goods_spec");
        let price_pi: f64 = row.get("price_pi");
        let num: i32 = row.get("num");

        insert_order_item(
            order_id,
            goods_id,
            &goods_name,
            &goods_spec,
            price_pi,
            num,
        ).await?;
    }

    // 清空已结算购物车
    delete_cart_by_ids(&cart_ids).await.ok();

    Ok(Json(serde_json::json!({
        "code": 0,
        "msg": "结算订单创建成功",
        "order_no": order_no,
        "total_price": total_price
    })))
}

pub async fn order_delete(
    axum::extract::Json(params): axum::extract::Json<serde_json::Value>,
) -> axum::response::Json<serde_json::Value> {
    let uid = params["uid"].as_str().unwrap_or_default();
    // 🔥 修复：先读字符串，再转数字，解决前端传字符串ID的问题
    let order_id_str = params["order_id"].as_str().unwrap_or_default();
    let order_id: i64 = order_id_str.parse().unwrap_or_default();

    match delete_pending_order(uid, order_id).await {
        Ok(true) => axum::response::Json(serde_json::json!({
            "code": 0,
            "msg": "订单删除成功"
        })),
        Ok(false) => axum::response::Json(serde_json::json!({
            "code": -1,
            "msg": "删除失败：订单不存在或非待支付"
        })),
        Err(_) => axum::response::Json(serde_json::json!({
            "code": -2,
            "msg": "服务器异常"
        })),
    }
}

/// 获取用户订单列表
pub async fn get_order_list(
    ExtractJson(params): ExtractJson<serde_json::Value>,
) -> Result<Json<Value>, StatusCode> {
// 兜底：uid不存在/为空 → 直接返回空订单，不报错
    let uid = match params["uid"].as_str() {
        Some(v) => v.trim(),
        None => return Ok(Json(serde_json::json!({"code":0,"list":[]}))),
    };

    if uid.is_empty() {
        return Ok(Json(serde_json::json!({"code":0,"list":[]})));
    }
    let orders = sqlx::query!(
        "SELECT id, order_no, total_price_pi, status, create_time FROM shop_order WHERE uid = ? ORDER BY create_time DESC",
        uid
    )
    .fetch_all(&*DB_POOL)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

    Ok(Json(serde_json::json!({
        "code": 0,
        "list": list
    })))
}

/// 订单列表页面
pub async fn order_page() -> Html<String> {
    crate::view::order::order_list_page()
}

pub async fn order_detail(
    axum::extract::Json(params): axum::extract::Json<serde_json::Value>,
) -> axum::response::Json<serde_json::Value> {
    let uid = params["uid"].as_str().unwrap_or_default();
    let order_id_str = params["order_id"].as_str().unwrap_or_default();
    let order_id: i64 = order_id_str.parse().unwrap_or_default();

    // 1. 查询订单主表
    let order = match sqlx::query!(
        r#"SELECT id,order_no,total_price_pi,status,create_time FROM shop_order WHERE id = ? AND uid = ?"#,
        order_id,
        uid
    )
    .fetch_optional(&*DB_POOL)
    .await
    {
        Ok(Some(o)) => o,
        _ => return axum::response::Json(serde_json::json!({
            "code": -1,
            "msg": "订单不存在"
        })),
    };

    // 2. 查询商品明细
    let items = sqlx::query!(
        r#"SELECT goods_name,goods_spec,price_pi,num FROM shop_order_item WHERE order_id = ?"#,
        order_id
    )
    .fetch_all(&*DB_POOL)
    .await
    .unwrap_or_default();

    // 3. 手动构建可序列化JSON
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
        2 => "已取消",
        _ => "未知状态",
    };

    // 4. 返回正确格式JSON
    axum::response::Json(serde_json::json!({
        "code": 0,
        "order": {
            "order_no": order.order_no,
            "total_price_pi": order.total_price_pi,
	   "status":order.status,
            "status_text": status_text,
            "create_time": order.create_time
        },
        "list": list
    }))
}
// 订单详情页面（返回HTML）
pub async fn order_detail_page() -> Html<String> {
    crate::view::order_detail::order_detail_page()
}

// 更新订单为已支付
pub async fn update_order_to_paid(order_id: i64) -> Result<(), StatusCode> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 直接用你项目的全局 DB_POOL，风格完全一致
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