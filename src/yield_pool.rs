use solana_program::{
    program_error::ProgramError,
    account_info::AccountInfo,
    msg
};

use borsh::{BorshDeserialize};

use serum_pool::{context::PoolContext, schema::PoolState, pool::Pool};

use crate::{
    lending_protocols::{Oyster, LendingProtocol},
    state::{CustomPoolState, CustomPoolStateContainer, LendingProtocolState, YieldPoolTag}
};

pub struct YieldPool;

impl YieldPool {
    #[allow(non_snake_case)]
    pub fn optimise(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
        // TODO: write check
        // let account_info_iter = &mut accounts.iter();

        let oysterAPY = Oyster::calculate_deposit_APY(accounts);
        Ok(())
    }
}

// add struct for lending protocols

impl Pool for YieldPool {
    fn initialize_pool(context: &PoolContext, state: &mut PoolState) -> Result<(), ProgramError> {
        if context.custom_accounts.len() < 1 {
            msg!("Missing yield protocol accounts.");
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let lending_protocols = Vec::<LendingProtocolState>::deserialize(&mut &context.custom_data[..]).unwrap();

        state.write_custom_state(&CustomPoolState {
            tag: YieldPoolTag::default(),
            lending_protocols
        });

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
