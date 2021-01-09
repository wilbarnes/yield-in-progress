use solana_program::{
    program_error::ProgramError,
    account_info::AccountInfo,
    msg
};

use serum_pool::{context::PoolContext, schema::PoolState, pool::Pool};

use crate::{
    lending_protocols::{Oyster, LendingProtocol},
    state::{CustomPoolState, CustomPoolStateContainer}
};

pub struct YieldPool;

impl YieldPool {
    #[allow(non_snake_case)]
    pub fn optimise(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
        let oysterAPY = Oyster::calculate_deposit_APY(accounts);
        Ok(())
    }
}

impl Pool for YieldPool {
    fn initialize_pool(context: &PoolContext, state: &mut PoolState) -> Result<(), ProgramError> {
        if context.custom_accounts.len() < 1 {
            msg!("Missing yield protocol accounts.");
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        state.write_custom_state(&CustomPoolState::default())?;
        Ok(())
    }

    fn process_creation(
        context: &PoolContext,
        state: &mut PoolState,
        creation_size: u64,
    ) -> Result<(), ProgramError> {
        let basket = Self::get_creation_basket(context, state, creation_size)?;
        context.transfer_basket_from_user(&basket)?;
        context.mint_tokens(state, creation_size)?;
        YieldPool::optimise(context.custom_accounts)?;        
        Ok(())
    }
}
