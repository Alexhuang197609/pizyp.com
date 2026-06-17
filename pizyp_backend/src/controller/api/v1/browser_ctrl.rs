use axum::extract::Json;
use serde::Deserialize;
use crate::{
    db::get_pool,
    model::contract::Contract,
    model::contract_tx::ContractTx,
    utils::response::{success, fail}
};

#[derive(Deserialize)]
pub struct BrowserOverviewReq {
    pub contract_addr: String,
}

#[derive(Deserialize)]
pub struct TxDetailReq {
    pub tx_hash: String,
}

#[derive(Deserialize)]
pub struct AddressTxsReq {
    pub address: String,
}

pub async fn browser_overview(
    Json(req): Json<BrowserOverviewReq>
) -> Json<crate::utils::response::Res<serde_json::Value>> {
    let pool = get_pool();

    let row = match sqlx::query!(
        r#"
        SELECT id, name, desc, status, logo_blob, contract_addr
        FROM contract
        WHERE contract_addr = ?
        "#,
        req.contract_addr
    ).fetch_optional(pool).await {
        Ok(Some(r)) => r,
        Ok(None) => return fail("合约不存在"),
        Err(e) => return fail(&format!("查询失败: {}", e)),
    };

    let contract = Contract {
        id: row.id.unwrap(),
        name: row.name,
        desc: row.desc,
        status: row.status,
        logo_blob: row.logo_blob,
        contract_addr: row.contract_addr,
    };

    let tx_list = sqlx::query_as!(
        ContractTx,
        "SELECT * FROM contract_tx WHERE contract_addr = ? ORDER BY ledger DESC",
        req.contract_addr
    ).fetch_all(pool).await.unwrap_or_default();

    let resp_data = serde_json::json!({
        "contract": contract,
        "tx_list": tx_list
    });
    success(resp_data)
}

pub async fn tx_detail(
    Json(req): Json<TxDetailReq>
) -> Json<crate::utils::response::Res<serde_json::Value>> {
    let pool = get_pool();

    let tx = match sqlx::query!(
        r#"
        SELECT id, contract_addr, tx_hash, ledger, tx_time, from_addr, to_addr, amount, event_type, token_type
        FROM contract_tx
        WHERE tx_hash = ?
        "#,
        req.tx_hash
    ).fetch_optional(pool).await {
        Ok(Some(tx)) => tx,
        Ok(None) => return fail("交易不存在"),
        Err(e) => return fail(&format!("查询失败: {}", e)),
    };

    let resp_data = serde_json::json!({
        "tx_hash": tx.tx_hash,
        "ledger": tx.ledger,
        "tx_time": tx.tx_time,
        "from_addr": tx.from_addr,
        "to_addr": tx.to_addr,
        "amount": tx.amount,
        "event_type": tx.event_type,
        "token_type": tx.token_type
    });
    success(resp_data)
}

pub async fn address_txs(
    Json(req): Json<AddressTxsReq>
) -> Json<crate::utils::response::Res<serde_json::Value>> {
    let pool = get_pool();

    let tx_list = sqlx::query_as!(
        ContractTx,
        r#"
        SELECT 
            id, contract_addr, tx_hash, ledger, tx_time,
            from_addr, to_addr, amount, event_type, token_type
        FROM contract_tx
        WHERE from_addr = ? OR to_addr = ?
        ORDER BY ledger DESC
        "#,
        req.address,
        req.address
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let resp_data = serde_json::json!({
        "tx_list": tx_list
    });
    success(resp_data)
}