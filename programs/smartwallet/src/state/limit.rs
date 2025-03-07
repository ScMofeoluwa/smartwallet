use anchor_lang::prelude::*;

#[account]
pub struct Limit {
    pub total_limit: Option<u64>,
    pub tx_limit: Option<u64>,
}

