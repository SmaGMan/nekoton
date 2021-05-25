use serde::{Deserialize, Serialize};

use crate::core::models::TransactionId;
use crate::utils::*;

#[derive(Serialize, Debug, Clone)]
pub struct GetContractState {
    #[serde(with = "serde_address")]
    pub address: ton_block::MsgAddressInt,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendMessage<'a> {
    #[serde(with = "serde_message")]
    pub message: &'a ton_block::Message,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactions {
    #[serde(with = "serde_address")]
    pub address: ton_block::MsgAddressInt,
    pub transaction_id: TransactionId,
    pub count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTransactionsList {
    #[serde(with = "serde_bytes_base64")]
    pub transactions: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBlock {
    #[serde(with = "serde_ton_block")]
    pub block: ton_block::Block,
}