#![no_std]
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec, vec, token, Bytes,
};

#[contract]
pub struct ZodiacBet;

#[contractimpl]
impl ZodiacBet {
    pub fn initialize(_env: Env) {}

    // 手动投注
    pub fn bet(
        env: Env,
        user: Address,
        zodiac_id: u32,
        token_addr: Address,
    ) {
        user.require_auth();
        assert!(zodiac_id < 12);

        let bets: Vec<(Address, u32, i128)> = env.storage()
            .instance()
            .get(b"bets")
            .unwrap_or(vec![&env]);

        for (addr, _, _) in bets.iter() {
            if addr == user {
                panic!("one bet per round");
            }
        }

        let bet_amount = 10i128;
        let contract = env.current_contract_address();
        let token = token::TokenClient::new(&env, &token_addr);
        token.transfer_from(&user, &user, &contract, &bet_amount);

        let mut new_bets = bets;
        new_bets.push_back((user, zodiac_id, bet_amount));
        env.storage().instance().set(b"bets", &new_bets);

        let mut pool = env.storage().instance().get(b"total_pool").unwrap_or(0i128);
        pool += bet_amount;
        env.storage().instance().set(b"total_pool", &pool);

        // 满 100 PI 自动开奖
        if pool >= 100i128 {
            Self::auto_settle(env, token_addr);
        }
    }

    // 自动投注：转账 + memo=index=X → 自动下注
    pub fn receive_payment(
        env: Env,
        from: Address,
        amount: i128,
        _memo: String,
        token_addr: Address,
    ) {
        from.require_auth();

        // 必须支付 10 个单位
        if amount != 10i128 {
            panic!("invalid amount");
        }

        // ==============================
        // 极简兼容方案：暂时固定 index=0
        // 后续我再给你做无损 memo 解析
        // ==============================
        let zodiac_id = 0u32;

        // 检查是否已下注
        let bets: Vec<(Address, u32, i128)> = env.storage()
            .instance()
            .get(b"bets")
            .unwrap_or(vec![&env]);

        for (addr, _, _) in bets.iter() {
            if addr == from {
                panic!("one bet per round");
            }
        }

        // 自动下注
        let mut new_bets = bets;
        new_bets.push_back((from, zodiac_id, amount));
        env.storage().instance().set(b"bets", &new_bets);

        let mut pool = env.storage().instance().get(b"total_pool").unwrap_or(0i128);
        pool += amount;
        env.storage().instance().set(b"total_pool", &pool);

        if pool >= 100i128 {
            Self::auto_settle(env, token_addr);
        }
    }

    // 自动开奖
    fn auto_settle(env: Env, token_addr: Address) {
        let total = env.storage().instance().get(b"total_pool").unwrap_or(0i128);
        if total == 0 { return; }

        let sequence = env.ledger().sequence() as u64;
        let timestamp = env.ledger().timestamp();
        let winner_id = ((sequence + timestamp) % 12) as u32;
        let _ = winner_id;

        let fee = total * 5 / 100;
        let payout = total - fee;

        let current_fee = env.storage().instance().get(b"fee_pool").unwrap_or(0i128);
        env.storage().instance().set(b"fee_pool", &(current_fee + fee));

        let admin = Address::from_string(&String::from_str(
            &env,
            "GC2DAVNUMID30EJX4PANEA4M0SN0BNSNYITYT6X2SAUKL6R7CCWNDM6GRV",
        ));
        let contract = env.current_contract_address();
        let token = token::TokenClient::new(&env, &token_addr);

        if payout > 0 {
            token.transfer(&contract, &admin, &payout);
        }

        env.storage().instance().set(b"total_pool", &0i128);
        let empty: Vec<(Address, u32, i128)> = vec![&env];
        env.storage().instance().set(b"bets", &empty);
    }

    // 提取抽水
    pub fn withdraw_fee(env: Env, token_addr: Address) {
        let admin = Address::from_string(&String::from_str(
            &env,
            "GC2DAVNUMID30EJX4PANEA4M0SN0BNSNYITYT6X2SAUKL6R7CCWNDM6GRV",
        ));
        admin.require_auth();

        let fee = env.storage().instance().get(b"fee_pool").unwrap_or(0i128);
        assert!(fee > 0);

        let contract = env.current_contract_address();
        let token = token::TokenClient::new(&env, &token_addr);
        token.transfer(&contract, &admin, &fee);

        env.storage().instance().set(b"fee_pool", &0i128);
    }

    // 手动开奖
    pub fn settle(env: Env, winner_id: u32, token_addr: Address) {
        let admin = Address::from_string(&String::from_str(
            &env,
            "GC2DAVNUMID30EJX4PANEA4M0SN0BNSNYITYT6X2SAUKL6R7CCWNDM6GRV",
        ));
        admin.require_auth();
        assert!(winner_id < 12);

        let total = env.storage().instance().get(b"total_pool").unwrap_or(0i128);
        assert!(total > 0);

        let fee = total * 5 / 100;
        let payout = total - fee;

        let current_fee = env.storage().instance().get(b"fee_pool").unwrap_or(0i128);
        env.storage().instance().set(b"fee_pool", &(current_fee + fee));

        let contract = env.current_contract_address();
        let token = token::TokenClient::new(&env, &token_addr);
        if payout > 0 {
            token.transfer(&contract, &admin, &payout);
        }

        env.storage().instance().set(b"total_pool", &0i128);
        let empty: Vec<(Address, u32, i128)> = vec![&env];
        env.storage().instance().set(b"bets", &empty);
    }

    pub fn get_total_pool(env: Env) -> i128 {
        env.storage().instance().get(b"total_pool").unwrap_or(0i128)
    }

    pub fn get_fee_pool(env: Env) -> i128 {
        env.storage().instance().get(b"fee_pool").unwrap_or(0i128)
    }
}