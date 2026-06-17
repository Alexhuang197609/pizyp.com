use axum::Json as RespJson;
use axum::extract::Json as ReqJson;
use crate::db::DB_POOL;
use serde::{Serialize, Deserialize};
use base64::engine::{general_purpose, Engine as _};
use crate::utils::response::{success, fail, Res};

#[derive(Serialize)]
pub struct GoodsItem {
    id: i64,
    goods_name: String,
    goods_spec: String,
    img_base64: String,
    price_pi: f64,
    goods_stock: i32,
}

pub async fn shop_data() -> RespJson<Res<Vec<GoodsItem>>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, goods_name, goods_spec, goods_img_blob,
            price_pi, goods_stock, goods_desc,
            sort_num, is_on_shelf, create_time, update_time
        FROM shop_goods
        WHERE is_on_shelf = 1
        ORDER BY sort_num ASC
        "#
    )
    .fetch_all(&*DB_POOL)
    .await
    .unwrap_or_default();

    let mut list = Vec::new();
    for r in rows {
        let img_base64 = r.goods_img_blob
            .as_ref()
            .map(|b| general_purpose::STANDARD.encode(b))
            .unwrap_or_default();

        list.push(GoodsItem {
            id: r.id,
            goods_name: r.goods_name,
            goods_spec: r.goods_spec.unwrap_or_default(),
            img_base64,
            price_pi: r.price_pi as f64,
            goods_stock: r.goods_stock as i32,
        });
    }

    success(list)
}

#[derive(Serialize)]
pub struct GoodsDetailItem {
    id: i64,
    goods_name: String,
    goods_spec: String,
    goods_desc: String,
    goods_ship_addr: String,
    price_pi: f64,
    goods_stock: i32,
    goods_sales: i32,
    img1: String,
    img2: String,
    img3: String,
    d1: String,
    d2: String,
    d3: String,
    d4: String,
}

#[derive(Deserialize)]
pub struct DetailReq {
    id: i64,
}

pub async fn shop_detail(ReqJson(req): ReqJson<DetailReq>) -> RespJson<Res<GoodsDetailItem>> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, goods_name, goods_spec,
            goods_img1_blob, goods_img2_blob, goods_img3_blob,
            goods_detail1_blob, goods_detail2_blob, goods_detail3_blob, goods_detail4_blob,
            price_pi, goods_stock, goods_desc, goods_ship_addr, goods_sales,
            is_on_shelf
        FROM shop_goods
        WHERE id = ? AND is_on_shelf = 1
        "#,
        req.id
    )
    .fetch_all(&*DB_POOL)
    .await
    .unwrap_or_default();

    if rows.is_empty() {
        return fail("商品不存在或已下架");
    }
    let r = &rows[0];

    let encode = |opt: &Option<Vec<u8>>| -> String {
        opt.as_ref()
            .map(|b| general_purpose::STANDARD.encode(b))
            .unwrap_or_default()
    };

    let detail = GoodsDetailItem {
        id: r.id,
        goods_name: r.goods_name.clone(),
        goods_spec: r.goods_spec.clone().unwrap_or_default(),
        goods_desc: r.goods_desc.clone().unwrap_or_default(),
        goods_ship_addr: r.goods_ship_addr.clone().unwrap_or_default(),
        price_pi: r.price_pi as f64,
        goods_stock: r.goods_stock as i32,
        goods_sales: r.goods_sales as i32,
        img1: encode(&r.goods_img1_blob),
        img2: encode(&r.goods_img2_blob),
        img3: encode(&r.goods_img3_blob),
        d1: encode(&r.goods_detail1_blob),
        d2: encode(&r.goods_detail2_blob),
        d3: encode(&r.goods_detail3_blob),
        d4: encode(&r.goods_detail4_blob),
    };

    success(detail)
}