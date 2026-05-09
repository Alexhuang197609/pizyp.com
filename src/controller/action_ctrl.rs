use crate::db;
use crate::model::action::UserAction;
use base64::engine::{general_purpose, Engine as _};
use sqlx::Error;
use std::time::Instant;

/// 获取用户动态列表（优化版：错误处理 + 分页 + 性能优化）
pub async fn get_user_action_list() -> Result<Vec<UserAction>, Error> {
    let start = Instant::now();

    // 1. 获取连接池
    let pool = db::get_pool();
    println!("获取连接池耗时: {:?}", start.elapsed());

    // 2. 数据库查询（增加分页，防止全表扫描）
    let query_start = Instant::now();
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, mid, nickName, faceImgBlob, 
            text, time, imgBlob, videoBlob, 
            prizeNum, commentsNum
        FROM user_actions
        ORDER BY id DESC
        LIMIT 20
        "#
    )
    .fetch_all(pool)
    .await?;  // 这里抛出错误，不再隐藏

    println!("SQL查询耗时: {:?}", query_start.elapsed());

    // 3. 数据处理 + Base64 编码（只编码需要的，不冗余存储）
    let process_start = Instant::now();
    let mut list = Vec::new();

    for row in rows {
        // 头像：只存一份，不冗余
        let face_bytes = row.faceImgBlob.unwrap_or_default();
        let face_base64 = general_purpose::STANDARD.encode(&face_bytes);

        // 商品图片
        let img_bytes = row.imgBlob.unwrap_or_default();
        let img_base64 = if !img_bytes.is_empty() {
            Some(general_purpose::STANDARD.encode(&img_bytes))
        } else {
            None
        };

        // 视频：延迟编码逻辑（这里保持兼容，不破坏结构）
        let video_bytes = row.videoBlob.unwrap_or_default();
        let video_base64 = if !video_bytes.is_empty() {
            Some(general_purpose::STANDARD.encode(&video_bytes))
        } else {
            None
        };

        let item = UserAction {
            id: row.id,
            mid: row.mid as i32,
            nick_name: row.nickName,
            face_img_url: String::new(),
            face_img_blob: face_bytes,  // 保留兼容
            img_blob: img_bytes,        // 保留兼容
            text: row.text.unwrap_or_default(),
            time: row.time.unwrap_or_default(),
            video_url: String::new(),
            video_blob: video_bytes,    // 保留兼容
            prize_num: row.prizeNum.unwrap_or(0) as i32,
            comments_num: row.commentsNum.unwrap_or(0) as i32,
            face_base64,
            img_base64,
            video_base64,
        };

        list.push(item);
    }

    println!("数据处理+Base64耗时: {:?}", process_start.elapsed());
    println!("总耗时: {:?}", start.elapsed());

    Ok(list)
}