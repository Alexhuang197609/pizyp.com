use axum::{routing::post, Router, serve};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

mod config;
mod utils;
mod model;
mod controller;
mod db;

use controller::api::v1::{
    action_ctrl,
    browser_ctrl,
    cart_ctrl,
    contract_ctrl,
    goods_ctrl,
    order_ctrl,
    pi_ctrl,
    user_ctrl,
    zodiac_ctrl
};

fn api_v1_routes() -> Router {
    Router::new()
        .route("/api/v1/action/list", post(action_ctrl::action_list_handler))
        // 懒加载媒体接口 GET /api/v1/action/media?id=xxx
        .route("/api/v1/action/media", post(action_ctrl::action_media_handler))
        .route("/api/v1/shop/data", post(goods_ctrl::shop_data))
        .route("/api/v1/shop/detail", post(goods_ctrl::shop_detail))
        .route("/api/v1/cart/add", post(cart_ctrl::add_cart))
        .route("/api/v1/cart/list", post(cart_ctrl::cart_list))
        .route("/api/v1/cart/update-num", post(cart_ctrl::cart_update_num))
        .route("/api/v1/order/buy-now", post(order_ctrl::order_buy_now))
        .route("/api/v1/order/settle", post(order_ctrl::order_settle))
        .route("/api/v1/order/delete", post(order_ctrl::order_delete))
        .route("/api/v1/order/list", post(order_ctrl::get_order_list))
        .route("/api/v1/order/detail", post(order_ctrl::order_detail))
        .route("/api/v1/pi/verify", post(pi_ctrl::pi_verify))
        .route("/api/v1/pi/payments/:payment_id/approve", post(pi_ctrl::pi_payment_approve))
        .route("/api/v1/pi/payments/:payment_id/complete", post(pi_ctrl::pi_payment_complete))
        .route("/api/v1/contract/data", post(contract_ctrl::contract_data))
        .route("/api/v1/contract/tx-import", post(contract_ctrl::tx_import))
        .route("/api/v1/browser/overview", post(browser_ctrl::browser_overview))
        .route("/api/v1/browser/tx-detail", post(browser_ctrl::tx_detail))
        .route("/api/v1/browser/address-txs", post(browser_ctrl::address_txs))
        // 用户中心三条接口
        .route("/api/v1/my/data", post(user_ctrl::my_data))
        .route("/api/v1/my/bind-wallet", post(user_ctrl::bind_wallet))
        .route("/api/v1/my/bind-mini", post(user_ctrl::bind_mini_account))
        .route("/api/v1/contract/zodiac-bets", post(zodiac_ctrl::zodiac_bets_data))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    db::init_table().await.expect("数据库初始化失败");

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(|origin_val, _req| {
            let origin = origin_val.to_str().unwrap_or("");
            origin.is_empty() || origin == "https://pizyp.com" || origin.ends_with(".piappengine.com")
        }))
        .allow_methods(tower_http::cors::any())
        .allow_headers(tower_http::cors::any())
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .merge(api_v1_routes())
        .layer(cors);

    let listen_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum start: listen 127.0.0.1:3000");

    let listener = TcpListener::bind(listen_addr).await?;
    serve(listener, app.into_make_service()).await
}