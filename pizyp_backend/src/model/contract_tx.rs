use crate::model::contract::Contract;
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContractTx {
    pub id: i64,
    pub contract_addr: String,
    pub tx_hash: String,
    pub ledger: i64,
    pub tx_time: String,
    pub from_addr: String,
    pub to_addr: String,
    pub amount: String,
    pub event_type: String,
    pub token_type: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ContractDetail {
    pub contract: Contract,
    pub tx_list: Vec<ContractTx>,
}

#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GlobalSearchResult {
    pub contract: Option<Contract>,
    pub tx_detail: Option<ContractTx>,
    pub addr_tx_list: Option<Vec<ContractTx>>,
}