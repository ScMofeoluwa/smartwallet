use anchor_lang::prelude::*;

use crate::state::{Limit, Wallet};
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction()]
pub struct Settings<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds=[b"wallet", owner.key().as_ref()], bump=wallet.bump)]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> Settings<'info>{
    pub fn update_limit(&mut self, limit: Limit) -> Result<()>{
       if let Some(dl) = limit.total_limit {
           if dl == 0 {
               return err!(ErrorCode::InvalidLimit)
           }
           self.wallet.total_limit = dl
       }
       if let Some(dl) = limit.tx_limit {
           if dl == 0 {
               return err!(ErrorCode::InvalidLimit)
           }
           self.wallet.tx_limit = dl
       }
       Ok(())
    }
}
