use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserAction {
    pub id: i64,
    pub mid: i32,
    pub nick_name: String,

    pub face_img_url: String,
    pub face_img_blob: Vec<u8>,
   

    pub img_blob: Vec<u8>,
    pub text: String,
    pub time: String,


    pub video_url: String,
    pub video_blob: Vec<u8>,

    pub prize_num: i32,
    pub comments_num: i32,

    // 👇 以下是新增：用于前端渲染的 base64 字段（Controller 处理后赋值）
    pub face_base64: String,
    pub img_base64: Option<String>,
    pub video_base64: Option<String>,
}