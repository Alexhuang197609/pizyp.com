use serde::{Deserialize, Serialize};

/// 订单明细表 shop_order_item
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub goods_id: i64,
    pub goods_name: Option<String>,
    pub goods_spec: Option<String>,
    pub price_pi: f64,
    pub num: i32,
    pub create_time: String,
}