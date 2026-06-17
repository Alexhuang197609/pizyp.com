/// 商品模型 (与 shop_goods 表完全对应)
#[derive(Debug, Default)]
pub struct Goods {
    pub id: i64,
    pub goods_name: String,
    pub goods_spec: Option<String>,
    pub goods_img_blob: Option<Vec<u8>>,
    pub goods_img1_blob: Option<Vec<u8>>,
    pub goods_img2_blob: Option<Vec<u8>>,
    pub goods_img3_blob: Option<Vec<u8>>,
    pub goods_detail1_blob: Option<Vec<u8>>,
    pub goods_detail2_blob: Option<Vec<u8>>,
    pub goods_detail3_blob: Option<Vec<u8>>,
    pub goods_detail4_blob: Option<Vec<u8>>,
    pub price_pi: f64,
    pub goods_stock: i32,
    pub goods_desc: Option<String>,
    pub goods_ship_addr: Option<String>,
    pub goods_sales: i32,
    pub sort_num: i32,
    pub is_on_shelf: i32,
    pub create_time: String,
    pub update_time: String,
}