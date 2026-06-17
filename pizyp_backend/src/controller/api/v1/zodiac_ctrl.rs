use axum::Json;
use serde::Serialize;
use serde_json::json;
use crate::utils::response::{success, fail, Res};

// 前端接口结构
#[derive(Serialize)]
pub struct ZodiacMockResp {
    pub bets: Vec<u32>,
    pub pool: f64,
    pub user_num: u32,
    pub lastOpen: String,
    pub need: u32,
}

// 真实合约读取接口
pub async fn zodiac_bets_data() -> Json<Res<ZodiacMockResp>> {
    const CONTRACT_ID: &str = "CD2EIQWTDKLX2X7MP5Q37WYNHHZDNQA33GZDE3TVVBZKATRBKYSHOCI5";
    const RPC_URL: &str = "https://soroban-testnet.stellar.org:443";
    const TARGET_PLAYERS: u32 = 100;

    let client = reqwest::Client::new();

    // 1. 读取当期总投注人数
    let total_players = call_contract(
        &client,
        RPC_URL,
        CONTRACT_ID,
        "get_total_players",
        &[]
    ).await.unwrap_or(0);

    // 2. 读取当期总奖池
    let total_pool = call_contract(
        &client,
        RPC_URL,
        CONTRACT_ID,
        "get_total_pool",
        &[]
    ).await.unwrap_or(0);

    // 3. 读取12生肖各自投注人数
    let mut bets = vec![0; 12];
    for i in 0..12 {
        bets[i] = call_contract(
            &client,
            RPC_URL,
            CONTRACT_ID,
            "get_bet_count",
            &[i as u32]
        ).await.unwrap_or(0);
    }

    // 4. 读取上期开奖生肖
    let last_zodiac = call_contract(
        &client,
        RPC_URL,
        CONTRACT_ID,
        "get_last_zodiac",
        &[]
    ).await.unwrap_or(0);

    let zodiac_names = [
        "子鼠", "丑牛", "寅虎", "卯兔", "辰龙", "巳蛇",
        "午马", "未羊", "申猴", "酉鸡", "戌狗", "亥猪"
    ];
    let lastOpen = zodiac_names.get(last_zodiac as usize)
        .unwrap_or(&"未知")
        .to_string();

    // 5. 计算还差多少人开奖
    let need = if total_players >= TARGET_PLAYERS {
        0
    } else {
        TARGET_PLAYERS - total_players
    };

    let resp_data = ZodiacMockResp {
        bets,
        pool: total_pool as f64 / 10.0,
        user_num: total_players,
        lastOpen,
        need,
    };
    success(resp_data)
}

// 核心：原生 HTTP 调用 Soroban 合约（逻辑完全不变）
async fn call_contract(
    client: &reqwest::Client,
    rpc: &str,
    contract: &str,
    func: &str,
    args: &[u32]
) -> Option<u32> {
    let mut params_args = vec![];
    for &v in args {
        params_args.push(json!({
            "type": "u32",
            "value": v.to_string()
        }));
    }

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "invoke",
        "params": {
            "contractId": contract,
            "function": func,
            "args": params_args
        }
    });

    let resp = client
        .post(rpc)
        .json(&body)
        .send()
        .await
        .ok()?;

    let json: serde_json::Value = resp.json().await.ok()?;
    let val = json["result"]["value"].as_str()?;
    val.parse().ok()
}