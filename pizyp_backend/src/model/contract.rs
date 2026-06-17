use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub status: String,
    pub logo_blob: Option<Vec<u8>>,
    pub contract_addr: Option<String>,
}