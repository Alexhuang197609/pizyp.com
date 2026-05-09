use axum::{response::Html, extract::Path, Json, response::IntoResponse};
use crate::db::DB_POOL;
use crate::model::goods::Goods;
use crate::view::{render_shop_page, render_goods_detail_page};
use serde::Serialize;
use base64::engine::{general_purpose, Engine as _};

// 商城列表页：直接返回纯静态壳，不再查库
pub async fn shop_list_page() -> Html<String> {
    let html = render_shop_page().await;
    Html(html)
}

// 商品列表数据接口
#[derive(Serialize)]
pub struct ShopListResp {
    code: i32,
    list: Vec<GoodsItem>,
}

#[derive(Serialize)]
pub struct GoodsItem {
    id: i64,
    goods_name: String,
    goods_spec: String,
    img_base64: String,
    price_pi: f64,
    goods_stock: i32,
}

pub async fn shop_data() -> impl IntoResponse {
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

    Json(ShopListResp {
        code: 0,
        list,
    })
}

// 商品详情页（完全不动）
pub async fn goods_detail_page(Path(goods_id): Path<i64>) -> Html<String> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, goods_name, goods_spec, goods_img_blob,
            goods_img1_blob, goods_img2_blob, goods_img3_blob,
            goods_detail1_blob, goods_detail2_blob, goods_detail3_blob, goods_detail4_blob,
            price_pi, goods_stock, goods_desc, goods_ship_addr,
            goods_sales, sort_num, is_on_shelf, create_time, update_time
        FROM shop_goods
        WHERE id = ? AND is_on_shelf = 1
        "#,
        goods_id
    )
    .fetch_all(&*DB_POOL)
    .await
    .unwrap_or_default();

    if rows.is_empty() {
        return Html("<h1>商品不存在或已下架</h1><a href=\"/shop\">返回商城</a>".to_string());
    }

    let r = &rows[0];
    let goods = Goods {
        id: r.id,
        goods_name: r.goods_name.clone(),
        goods_spec: r.goods_spec.clone(),
        goods_img_blob: r.goods_img_blob.clone(),
        goods_img1_blob: r.goods_img1_blob.clone(),
        goods_img2_blob: r.goods_img2_blob.clone(),
        goods_img3_blob: r.goods_img3_blob.clone(),
        goods_detail1_blob: r.goods_detail1_blob.clone(),
        goods_detail2_blob: r.goods_detail2_blob.clone(),
        goods_detail3_blob: r.goods_detail3_blob.clone(),
        goods_detail4_blob: r.goods_detail4_blob.clone(),
        price_pi: r.price_pi as f64,
        goods_stock: r.goods_stock as i32,
        goods_desc: r.goods_desc.clone(),
        goods_ship_addr: r.goods_ship_addr.clone(),
        goods_sales: r.goods_sales as i32,
        sort_num: r.sort_num as i32,
        is_on_shelf: r.is_on_shelf as i32,
        create_time: r.create_time.clone(),
        update_time: r.update_time.clone(),
        ..Default::default()
    };

    let html = render_goods_detail_page(&goods);
    Html(html)
}