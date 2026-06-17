use crate::db;
use crate::model::action::UserAction;
use base64::engine::{general_purpose, Engine as _};
use sqlx::Error;
use std::time::Instant;
use crate::utils::response::{success, fail, Res};
use axum::Json;
use serde::{Serialize, Deserialize};

// POST媒体接口请求体
#[derive(Deserialize)]
pub struct ActionMediaBody {
    pub id: i64,
}

// 列表轻量化返回结构体（无二进制Blob、无Base64）
#[derive(Serialize)]
pub struct UserActionBrief {
    pub id: i64,
    pub mid: i32,
    pub nick_name: String,
    pub text: String,
    pub time: String,
    pub prize_num: i32,
    pub comments_num: i32,
}

// 懒加载媒体返回结构体（仅存放各类Base64）
#[derive(Serialize)]
pub struct UserActionMedia {
    pub face_base64: String,
    pub img_base64: Option<String>,
    pub video_base64: Option<String>,
}

/// 轻量化动态列表接口 POST /api/v1/action/list
pub async fn action_list_handler() -> Json<Res<Vec<UserActionBrief>>> {
    let list = match get_user_action_brief_list().await {
        Ok(list) => success(list),
        Err(e) => {
            eprintln!("查询动态列表失败:{:?}", e);
            fail("查询动态数据失败")
        }
    };
    list
}

/// 单条媒体Base64获取接口 POST /api/v1/action/media
pub async fn action_media_handler(Json(params): Json<ActionMediaBody>) -> Json<Res<UserActionMedia>> {
    match get_single_action_with_base64(params.id).await {
        Ok(media) => success(media),
        Err(e) => {
            eprintln!("查询媒体Base64失败:{:?}", e);
            fail("获取媒体资源失败")
        }
    }
}

/// 查询精简列表（只拿文本ID基础字段，不读取Blob）
pub async fn get_user_action_brief_list() -> Result<Vec<UserActionBrief>, Error> {
    let start = Instant::now();
    let pool = db::get_pool();

    let rows = sqlx::query!(
        r#"
        SELECT 
            id, mid, nickName, text, time, prizeNum, commentsNum
        FROM user_actions
        ORDER BY id DESC
        LIMIT 20
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut list = Vec::new();
    for row in rows {
        list.push(UserActionBrief {
            id: row.id,
            mid: row.mid as i32,
            nick_name: row.nickName,
            text: row.text.unwrap_or_default(),
            time: row.time.unwrap_or_default(),
            prize_num: row.prizeNum.unwrap_or(0) as i32,
            comments_num: row.commentsNum.unwrap_or(0) as i32,
        });
    }

    println!("列表查询总耗时: {:?}", start.elapsed());
    Ok(list)
}

/// 根据ID单查Blob并实时转Base64（懒加载专用）
pub async fn get_single_action_with_base64(target_id: i64) -> Result<UserActionMedia, Error> {
    let pool = db::get_pool();
    let row = sqlx::query!(
        r#"
        SELECT faceImgBlob, imgBlob, videoBlob
        FROM user_actions
        WHERE id = ?
        "#,
        target_id
    )
    .fetch_one(pool)
    .await?;

    // 头像Base64
    let face_bytes = row.faceImgBlob.unwrap_or_default();
    let face_base64 = general_purpose::STANDARD.encode(&face_bytes);

    // 图片Base64
    let img_bytes = row.imgBlob.unwrap_or_default();
    let img_base64 = if !img_bytes.is_empty() {
        Some(general_purpose::STANDARD.encode(&img_bytes))
    } else {
        None
    };

    // 视频Base64
    let video_bytes = row.videoBlob.unwrap_or_default();
    let video_base64 = if !video_bytes.is_empty() {
        Some(general_purpose::STANDARD.encode(&video_bytes))
    } else {
        None
    };

    Ok(UserActionMedia {
        face_base64,
        img_base64,
        video_base64,
    })
}