use axum::{Json};
use base64::engine::{general_purpose::STANDARD, Engine as _};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use stellar_strkey::Strkey;
use stellar_xdr::curr::{ContractEvent, ContractEventBody, Limits, ScAddress, ScVal, ReadXdr, PublicKey, PublicKeyType};
use crc16::State;

use crate::db;
use crate::model::contract::Contract;
use crate::model::contract_tx::ContractTx;
use crate::utils::response::{success, fail, Res};

const ADMIN_ADDR: &str = "GC2DAVNUMID3OEJX4PANEA4MOSWOBNSYITYT6X2UKL6R7COWDM6GRV";
const TARGET_CONTRACT: &str = "CBHVUWXG4SF6WGKVZDLZT266EX2UH2K5D2UFZX64IACUI7V5KAWZ73FR";
const INIT_TX_HASH: &str = "9a798a937c108545ea536d7fe3ad619ce025712629916b2f4a8ba8d996ac7c";

#[derive(Serialize)]
pub struct ContractItemResp {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub status: String,
    pub contract_addr: Option<String>,
    pub logo_base64: Option<String>,
}

#[derive(Deserialize)]
pub struct TxImportReq {
    pub tx_hash: String,
}

async fn get_contract_list() -> Result<Vec<Contract>, Error> {
    let pool = db::get_pool();
    let rows = sqlx::query!(
        "SELECT id, name, desc, status, logo_blob, contract_addr FROM contract ORDER BY id ASC"
    )
    .fetch_all(pool)
    .await?;

    let mut list = Vec::new();
    for row in rows {
        list.push(Contract {
            id: row.id,
            name: row.name,
            desc: row.desc,
            status: row.status,
            logo_blob: row.logo_blob,
            contract_addr: row.contract_addr,
        });
    }
    Ok(list)
}

pub async fn contract_data() -> Json<Res<Vec<ContractItemResp>>> {
    let list = match get_contract_list().await {
        Ok(v) => v,
        Err(_) => return fail("加载合约列表失败"),
    };

    let resp_list = list
        .into_iter()
        .map(|c| {
            let logo_base64 = c.logo_blob.as_ref().map(|b| STANDARD.encode(b));
            ContractItemResp {
                id: c.id,
                name: c.name.clone(),
                desc: c.desc.clone(),
                status: c.status.clone(),
                contract_addr: c.contract_addr.clone(),
                logo_base64,
            }
        })
        .collect();

    success(resp_list)
}

pub async fn tx_import(Json(req): Json<TxImportReq>) -> Json<Res<()>> {
    let tx_hash = req.tx_hash.trim().to_string();

    if tx_hash.is_empty() || tx_hash.len() < 30 {
        return fail("非法交易哈希");
    }

    let rpc_data = match fetch_safe(&tx_hash).await {
        Err(err) => return fail("获取链上交易信息失败"),
        Ok(d) => d,
    };

    if rpc_data.status.as_str() != "SUCCESS" {
        return fail("链上交易执行未成功");
    }

    let (event_type, amount, from, to) = if tx_hash == INIT_TX_HASH {
        ("initialize".into(), "100000000000000".into(), ADMIN_ADDR.into(), ADMIN_ADDR.into())
    } else {
        ("transfer".into(), rpc_data.amount, rpc_data.from_addr, rpc_data.to_addr)
    };

    let tx = ContractTx {
        id: 0,
        contract_addr: TARGET_CONTRACT.into(),
        tx_hash,
        ledger: rpc_data.ledger,
        tx_time: rpc_data.tx_time,
        from_addr: from,
        to_addr: to,
        amount,
        event_type,
        token_type: "SUPI".into(),
    };

    match insert_tx(&tx).await {
        Ok(_) => success(()),
        Err(_) => fail("数据库写入失败"),
    }
}

pub async fn tx_query(Json(req): Json<TxImportReq>) -> Json<Res<Option<ContractTx>>> {
    let tx_hash = req.tx_hash.trim().to_string();

    if tx_hash.is_empty() || tx_hash.len() < 30 {
        return fail("非法哈希");
    }

    match get_tx_by_hash(&tx_hash).await {
        Ok(Some(tx)) => success(Some(tx)),
        Ok(None) => {
            let rpc_data = match fetch_safe(&tx_hash).await {
                Err(_) => return fail("链上查询失败"),
                Ok(d) => d,
            };

            if rpc_data.status != "SUCCESS" {
                return fail("交易执行失败");
            }

            let (event_type, amount, from, to) = if tx_hash == INIT_TX_HASH {
                ("initialize".into(), "100000000000000".into(), ADMIN_ADDR.into(), ADMIN_ADDR.into())
            } else {
                ("transfer".into(), rpc_data.amount, rpc_data.from_addr, rpc_data.to_addr)
            };

            let new_tx = ContractTx {
                id: 0,
                contract_addr: TARGET_CONTRACT.into(),
                tx_hash: tx_hash.clone(),
                ledger: rpc_data.ledger,
                tx_time: rpc_data.tx_time,
                from_addr: from,
                to_addr: to,
                amount,
                event_type,
                token_type: "SUPI".into(),
            };

            let _ = insert_tx(&new_tx).await;
            success(Some(new_tx))
        }
        Err(_) => fail("数据库查询出错"),
    }
}

async fn get_tx_by_hash(tx_hash: &str) -> Result<Option<ContractTx>, Error> {
    let pool = db::get_pool();
    let row = sqlx::query!(
        "SELECT id, contract_addr, tx_hash, ledger, tx_time, from_addr, to_addr, amount, event_type, token_type FROM contract_tx WHERE tx_hash = ? LIMIT 1",
        tx_hash
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ContractTx {
        id: r.id.unwrap_or_default(),
        contract_addr: r.contract_addr,
        tx_hash: r.tx_hash,
        ledger: r.ledger,
        tx_time: r.tx_time,
        from_addr: r.from_addr,
        to_addr: r.to_addr,
        amount: r.amount,
        event_type: r.event_type,
        token_type: r.token_type,
    }))
}

#[derive(Debug)]
struct SafeRpcData {
    status: String,
    ledger: i64,
    tx_time: String,
    from_addr: String,
    to_addr: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct RpcRootResp {
    result: Option<RpcTxResult>,
}

#[derive(Debug, Deserialize)]
struct RpcTxResult {
    events: EventsWrapper,
    ledger: i64,
}

#[derive(Debug, Deserialize)]
struct EventsWrapper {
    contractEventsXdr: Vec<Vec<String>>,
}

async fn fetch_safe(tx_hash: &str) -> Result<SafeRpcData, &'static str> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": {"hash":tx_hash}
    });

    let resp = client
        .post("https://rpc.testnet.minepi.com")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| "rpc_request_err")?;

    let resp_text = resp.text().await
        .map_err(|_| "rpc_read_body_err")?;

    let root: RpcRootResp = serde_json::from_str(&resp_text)
        .map_err(|_| "rpc_json_parse_err")?;

    let tx_result = root.result.ok_or("tx_not_exist")?;
    let xdr_list = &tx_result.events.contractEventsXdr;

    let xdr_base64 = xdr_list
        .first()
        .and_then(|g| g.first())
        .ok_or("no_contract_event")?;

    let (from_addr, to_addr, amount) = decode_transfer_xdr(TARGET_CONTRACT, xdr_base64)?;

    Ok(SafeRpcData {
        status: "SUCCESS".into(),
        ledger: tx_result.ledger,
        tx_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        from_addr,
        to_addr,
        amount,
    })
}

async fn insert_tx(tx: &ContractTx) -> Result<(), Error> {
    let pool = db::get_pool();
    sqlx::query(
        "INSERT INTO contract_tx (
            contract_addr, tx_hash, ledger, tx_time,
            from_addr, to_addr, amount, event_type, token_type
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&tx.contract_addr)
    .bind(&tx.tx_hash)
    .bind(tx.ledger)
    .bind(&tx.tx_time)
    .bind(&tx.from_addr)
    .bind(&tx.to_addr)
    .bind(&tx.amount)
    .bind(&tx.event_type)
    .bind(&tx.token_type)
    .execute(pool)
    .await?;

    Ok(())
}

fn decode_transfer_xdr(
    contract_hex: &str,
    base64_str: &str,
) -> Result<(String, String, String), &'static str> {
    let raw_bin = STANDARD.decode(base64_str)
        .map_err(|_| "decode_base64_err")?;

    let event = ContractEvent::from_xdr(&raw_bin, Limits::none())
        .map_err(|_| "xdr_parse_err")?;

    let mut from_addr = String::new();
    let mut to_addr = String::new();
    let mut amount = "0".to_string();

    if let ContractEventBody::V0(body) = event.body {
        match body.data {
            ScVal::I128(v) => amount = v.lo.to_string(),
            ScVal::U64(v) => amount = v.to_string(),
            _ => {}
        }

        if let ScVal::Address(ScAddress::Account(account_id)) = &body.topics[1] {
            if let PublicKey::PublicKeyTypeEd25519(pk) = &account_id.0 {
                from_addr = encode_stellar_address(&pk.0);
            }
        }

        if let ScVal::Address(ScAddress::Account(account_id)) = &body.topics[2] {
            if let PublicKey::PublicKeyTypeEd25519(pk) = &account_id.0 {
                to_addr = encode_stellar_address(&pk.0);
            }
        }
    }

    Ok((from_addr, to_addr, amount))
}

fn encode_stellar_address(pubkey: &[u8; 32]) -> String {
    use crc16::{State, XMODEM};

    let mut bytes = [0u8; 35];
    bytes[0] = 0x30;
    bytes[1..33].copy_from_slice(pubkey);

    let crc = State::<XMODEM>::calculate(&bytes[0..33]);
    bytes[33..].copy_from_slice(&crc.to_le_bytes());

    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = [0u8; 56];
    let mut i = 0;
    let mut idx = 0;

    while idx < 35 {
        let a = if idx < 35 { bytes[idx] } else { 0 };
        let b = if idx+1 < 35 { bytes[idx+1] } else { 0 };
        let c = if idx+2 < 35 { bytes[idx+2] } else { 0 };
        let d = if idx+3 < 35 { bytes[idx+3] } else { 0 };
        let e = if idx+4 < 35 { bytes[idx+4] } else { 0 };

        out[i]   = ALPHABET[((a >> 3) & 0x1F) as usize];
        out[i+1] = ALPHABET[(((a << 2) | (b >> 6)) & 0x1F) as usize];
        out[i+2] = ALPHABET[((b >> 1) & 0x1F) as usize];
        out[i+3] = ALPHABET[(((b << 4) | (c >> 4)) & 0x1F) as usize];
        out[i+4] = ALPHABET[(((c << 1) | (d >> 7)) & 0x1F) as usize];
        out[i+5] = ALPHABET[((d >> 2) & 0x1F) as usize];
        out[i+6] = ALPHABET[(((d << 3) | (e >> 5)) & 0x1F) as usize];
        out[i+7] = ALPHABET[(e & 0x1F) as usize];

        i += 8;
        idx += 5;
    }

    unsafe { core::str::from_utf8_unchecked(&out) }.to_string()
}