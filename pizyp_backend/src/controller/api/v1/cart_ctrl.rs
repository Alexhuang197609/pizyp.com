use axum::{http::StatusCode, Json, extract::Json as ExtractJson};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::DB_POOL;
use crate::utils::response::{success, fail, Res};

/// 加入购物车参数
#[derive(Debug, Deserialize)]
pub struct CartAddParams {
    pub uid: String,
    pub goods_id: i64,
    pub goods_spec: Option<String>,
    pub num: Option<i32>,
}

/// 获取购物车列表参数
#[derive(Debug, Deserialize)]
pub struct CartUidParams {
    pub uid: String,
}

/// 购物车单项返回结构体
#[derive(Debug, Serialize)]
pub struct UserCartItem {
    pub id: i64,
    pub goods_id: i64,
    pub goods_name: String,
    pub goods_spec: String,
    pub price_pi: f64,
    pub num: i32,
}

/// 查询购物车是否已存在
pub async fn check_cart_exists(
    uid: &str,
    goods_id: i64,
    goods_spec: &str,
) -> Result<Option<i64>, StatusCode> {
    let row = sqlx::query(
        "SELECT id FROM cart 
         WHERE uid = ? AND goods_id = ? AND goods_spec = ?"
    )
        .bind(uid)
        .bind(goods_id)
        .bind(goods_spec)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cart_id = row.map(|r| r.get("id"));
    Ok(cart_id)
}

/// 新增购物车
pub async fn insert_cart(
    uid: &str,
    goods_id: i64,
    goods_name: &str,
    goods_spec: &str,
    price_pi: f64,
    num: i32,
) -> Result<(), StatusCode> {
    sqlx::query(
        "INSERT INTO cart (uid, goods_id, goods_name, goods_spec, price_pi, num) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
        .bind(uid)
        .bind(goods_id)
        .bind(goods_name)
        .bind(goods_spec)
        .bind(price_pi)
        .bind(num)
        .execute(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

/// 更新购物车数量
pub async fn update_cart_num(
    cart_id: i64,
    add_num: i32,
) -> Result<(), StatusCode> {
    sqlx::query("UPDATE cart SET num = num + ? WHERE id = ?")
        .bind(add_num)
        .bind(cart_id)
        .execute(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

/// 查询商品基础信息（兼容price_pi为空）
pub async fn get_goods_base_info(goods_id: i64) -> Result<(String, f64), StatusCode> {
    let row = sqlx::query("SELECT goods_name, price_pi FROM shop_goods WHERE id = ?")
        .bind(goods_id)
        .fetch_one(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let goods_name: String = row.get("goods_name");
    let price_pi: f64 = row.try_get("price_pi").unwrap_or(0.0);
    Ok((goods_name, price_pi))
}

/// 根据 uid 查询用户购物车列表
pub async fn get_user_cart_list(uid: &str) -> Result<Vec<UserCartItem>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT id, goods_id, goods_name, goods_spec, price_pi, num FROM cart WHERE uid = ?",
        uid
    )
        .fetch_all(&*DB_POOL)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let list = rows.into_iter()
        .map(|r| UserCartItem {
            id: r.id,
            goods_id: r.goods_id,
            goods_name: r.goods_name.unwrap_or_default(),
            goods_spec: r.goods_spec.unwrap_or_default(),
            price_pi: r.price_pi,
            num: r.num.unwrap_or(0) as i32,
        })
        .collect();

    Ok(list)
}

// 购物车修改数量入参
#[derive(Debug, Deserialize)]
pub struct CartUpdateNumParams {
    pub cart_id: i64,
    pub step: i32, // 1 加 / -1 减
}

/// 加入购物车接口Handler
pub async fn add_cart(
    ExtractJson(params): ExtractJson<CartAddParams>,
) -> Json<Res<serde_json::Value>> {
    let uid = params.uid.trim();
    let goods_id = params.goods_id;
    let goods_spec = params.goods_spec.unwrap_or_default();
    let num = params.num.unwrap_or(1);

    if uid.is_empty() || goods_id <= 0 || num <= 0 {
        return fail("参数非法");
    }

    let (goods_name, price_pi) = match get_goods_base_info(goods_id).await {
        Ok(v) => v,
        Err(_) => return fail("查询商品信息失败"),
    };
    let cart_id_opt = match check_cart_exists(uid, goods_id, &goods_spec).await {
        Ok(v) => v,
        Err(_) => return fail("校验购物车失败"),
    };

    let op_res = match cart_id_opt {
        Some(cart_id) => update_cart_num(cart_id, num).await,
        None => insert_cart(uid, goods_id, &goods_name, &goods_spec, price_pi, num).await,
    };

    match op_res {
        Ok(_) => success(serde_json::json!({})),
        Err(_) => fail("购物车操作失败"),
    }
}

/// 获取购物车列表接口Handler
pub async fn cart_list(
    ExtractJson(params): ExtractJson<CartUidParams>,
) -> Json<Res<Vec<UserCartItem>>> {
    let uid = params.uid.trim();
    if uid.is_empty() {
        return fail("用户ID不能为空");
    }

    match get_user_cart_list(uid).await {
        Ok(list) => success(list),
        Err(_) => fail("加载购物车失败"),
    }
}

/// 修改购物车数量接口Handler
pub async fn cart_update_num(
    Json(params): Json<CartUpdateNumParams>,
) -> Json<Res<i64>> {
    // 查出当前数量
    let row_data = match sqlx::query!("SELECT num FROM cart WHERE id = ?", params.cart_id)
        .fetch_one(&*DB_POOL)
        .await {
            Ok(r) => r,
            Err(_) => return fail("查询购物车条目失败"),
        };

    let current_num = row_data.num.unwrap_or(1);
    let step = params.step as i64;
    let mut new_num = current_num + step;
    if new_num < 1 {
        new_num = 1;
    }

    match sqlx::query!("UPDATE cart SET num = ? WHERE id = ?", new_num, params.cart_id)
        .execute(&*DB_POOL)
        .await {
            Ok(_) => success(new_num),
            Err(_) => fail("更新数量失败"),
        }
}