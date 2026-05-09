use std::path::PathBuf;

/// 获取项目根目录
pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// 拼接cert目录路径
pub fn cert_path(filename: &str) -> PathBuf {
    project_root().join("cert").join(filename)
}

/// 拼接静态资源路径
pub fn static_path(rel_path: &str) -> PathBuf {
    project_root().join(rel_path)
}