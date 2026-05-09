use serde::{Deserialize, Serialize};

/// 订单主表 shop_order
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub uid: String,
    pub order_no: String,
    pub total_price_pi: f64,
    pub status: i32, // 0=待支付 1=已支付 2=已完成
    pub create_time: String,
    pub update_time: String,
}

/// 订单创建（前端提交）
#[derive(Debug, Deserialize)]
pub struct OrderCreateReq {
    pub uid: String,
}

/// 订单状态常量
pub const ORDER_STATUS_PENDING: i32 = 0; // 待支付
pub const ORDER_STATUS_PAID: i32 = 1;    // 已支付
pub const ORDER_STATUS_FINISHED: i32 = 2; // 已完成