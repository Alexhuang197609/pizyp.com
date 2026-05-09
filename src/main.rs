use axum::{routing::get, routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

mod config;
mod utils;
mod model;
mod controller;
mod view;
mod db;

use controller::{web_ctrl, static_ctrl, pi_ctrl, goods_ctrl, contract_ctrl, user_ctrl, cart_ctrl, order_ctrl};
use config::*;
use utils::path_util;
use tower_http::cors::{CorsLayer, Any};

/// 页面路由（所有页面GET）
fn page_routes() -> Router {
    Router::new()
        .route("/", get(web_ctrl::index))
        .route("/my", get(user_ctrl::my_page))
        .route("/shop", get(goods_ctrl::shop_list_page))
        .route("/goods/:id", get(goods_ctrl::goods_detail_page))
        .route("/cart", get(cart_ctrl::cart_page))
        .route("/order", get(order_ctrl::order_page))
        .route("/order/detail", get(order_ctrl::order_detail_page))
        .route("/contract", get(contract_ctrl::contract_page))
}

/// 接口路由（所有POST接口）
fn api_routes() -> Router {
    Router::new()
        // 首页
        .route("/api/home/data", post(web_ctrl::home_data))
        // 商城
        .route("/api/shop/data", post(goods_ctrl::shop_data))
        // 购物车
        .route("/api/cart/add", post(cart_ctrl::add_cart))
        .route("/api/cart/list", post(cart_ctrl::cart_list))
        .route("/api/cart/update-num", post(cart_ctrl::cart_update_num))
        // 订单
        .route("/api/order/buy-now", post(order_ctrl::order_buy_now))
        .route("/api/order/settle", post(order_ctrl::order_settle))
        .route("/api/order/delete", post(order_ctrl::order_delete))
        .route("/api/order/list", post(order_ctrl::get_order_list))
        .route("/api/order/detail", post(order_ctrl::order_detail))
        // PI支付
        .route("/api/pi/verify", post(pi_ctrl::pi_verify))
        .route("/api/pi/payments/:payment_id/approve", post(pi_ctrl::pi_payment_approve))
        .route("/api/pi/payments/:payment_id/complete", post(pi_ctrl::pi_payment_complete))
       //用户
      .route("/api/my/data", post(user_ctrl::my_data))
}

/// 静态&特殊路由
fn static_routes() -> Router {
    Router::new()
        .route("/static/banner1.jpg", get(static_ctrl::banner1))
        .route("/static/banner2.jpg", get(static_ctrl::banner2))
        .route("/static/banner3.jpg", get(static_ctrl::banner3))
        .route("/static/qrcode.jpg", get(static_ctrl::qrcode))
        .route("/validation-key.txt", get(web_ctrl::validation_key))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化数据库
    db::init_table().await.expect("数据库初始化失败");

    // 加载SSL证书
    let cert_path = path_util::cert_path(CERT_PEM_NAME);
    let key_path = path_util::cert_path(CERT_KEY_NAME);
    let tls_config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .expect("加载SSL证书失败");

    // CORS跨域
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 合并所有路由
    let app = Router::new()
        .merge(page_routes())
        .merge(api_routes())
        .merge(static_routes())
        .layer(cors);

    // 80端口重定向
    let redirect_app = Router::new().fallback(web_ctrl::redirect_to_https);

    let addr = SocketAddr::from(HTTPS_ADDR);
    let addr1 = SocketAddr::from(HTTP_ADDR);

    println!("HTTPS 已启动，监听 443 端口");

    let s1 = axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service());
    let s2 = axum_server::bind(addr1).serve(redirect_app.into_make_service());

    let _ = tokio::join!(s1, s2);
    Ok(())
}