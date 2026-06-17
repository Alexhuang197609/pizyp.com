use once_cell::sync::Lazy;
use sqlx::SqlitePool;

// 全局数据库连接池
pub static DB_POOL: Lazy<SqlitePool> = Lazy::new(|| {
    // 直接用 SqlitePool::connect_lazy，不用 SqlitePoolOptions
    SqlitePool::connect_lazy(crate::config::DB_URL)
        .expect("数据库连接失败")
});


// 初始化建表（程序启动时调用一次即可）
pub async fn init_table() -> Result<(), sqlx::Error> {
    let sql = r#"
CREATE TABLE IF NOT EXISTS pi_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pi_uid TEXT NOT NULL UNIQUE,
    username TEXT,
    invite_code TEXT UNIQUE,
    verify_code TEXT,
    token_claimed INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login_at DATETIME
);
CREATE TABLE IF NOT EXISTS shop_goods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    goods_name TEXT NOT NULL,
    goods_spec TEXT,
    goods_img_blob BLOB,
    goods_img1_blob BLOB,
    goods_img2_blob BLOB,
    goods_img3_blob BLOB,
    goods_detail1_blob BLOB,
    goods_detail2_blob BLOB,
    goods_detail3_blob BLOB,
    goods_detail4_blob BLOB,
    price_pi DECIMAL(20,18) NOT NULL DEFAULT 0.000000000000000000,
    goods_stock INTEGER NOT NULL DEFAULT 0,
    goods_desc TEXT,
    goods_ship_addr TEXT,
    goods_sales INTEGER NOT NULL DEFAULT 0,
    sort_num INTEGER NOT NULL DEFAULT 0,
    is_on_shelf INTEGER NOT NULL DEFAULT 1,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_pi_uid ON pi_users(pi_uid);
CREATE INDEX IF NOT EXISTS idx_verify_code ON pi_users(verify_code);
    "#;
    sqlx::query(sql).execute(&*DB_POOL).await?;
    Ok(())
}

pub fn get_pool() -> &'static SqlitePool {
    &DB_POOL
}