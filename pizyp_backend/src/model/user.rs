use serde::Serialize;

/// 派之优品用户表 (pi_users) 最终模型
#[derive(Debug, Clone, Serialize)]
pub struct User {
    // 基础字段
    pub id: i64,
    pub pi_uid: String,
    pub username: Option<String>,
    pub invite_code: Option<String>,
    pub verify_code: Option<String>,
    pub token_claimed: i32, // 0/1 标记

    // 扩展资料
    pub avatar: Option<Vec<u8>>, // BLOB 对应 Rust 字节数组
    pub nickname: Option<String>,
    pub gender: Option<i32>,
    pub bio: Option<String>,

    // 联系方式
    pub phone: Option<String>,
    pub email: Option<String>,

    // 业务核心
    pub private_key: Option<String>, // 密钥：校验小程序积分(mini openid)
    pub upi_num: f64,                // 小程序积分（4位小数）

    // 状态与权限
    pub status: i32,        // 1=正常 0=禁用
    pub user_type: i32,     // 0=普通用户 1=管理员
    pub site_code: String,  // 站点标识: hq=总部, zj=浙江, ah=安徽...

    // 钱包地址（表已新增列）
    pub wallet_address: Option<String>,

    // 时间字段（数据库原始字符串，不再转NaiveDateTime）
    pub created_at: String,
    pub updated_at: String,
    pub last_login_at: Option<String>,
}