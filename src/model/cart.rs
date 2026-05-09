/// 购物车实体 (对应 cart 表)
#[derive(Debug, Clone)]
pub struct Cart {
    pub id: i64,
    pub uid: String,
    pub goods_id: i64,
    pub goods_name: String,
    pub goods_spec: String,
    pub price_pi: f64,
    pub num: i32,
    pub create_time: String,
    pub update_time: String,
}

/// 加入购物车请求参数
#[derive(serde::Deserialize)]
pub struct AddCartRequest {
    pub uid: String,
    pub goods_id: i64,
    pub goods_spec: Option<String>,
    pub num: Option<i32>,
}