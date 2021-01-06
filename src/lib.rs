use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};
use serum_pool::schema::{
    declare_tag, AssetInfo, Basket, PoolState, FEE_RATE_DENOMINATOR, MIN_FEE_RATE, Address,
};
use serum_pool::{declare_pool_entrypoint, Pool, PoolContext};
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};

declare_tag!(YieldInstructionTag, u64, 0x31e6452361a17878);

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct YieldInstruction {
    tag: YieldInstructionTag,
    inner: YieldInstructionInner,
}

// NOTE: how to dynamically add new lending pools?

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub enum YieldInstructionInner {
    /// Rebalance between lenders
    ///
    /// Accounts:
    ///
	Rebalance,
}

// NOTE: why do we need this?
declare_tag!(YieldPoolTag, u64, 0x4a3ab7f76f93f94e);

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Default)]
pub struct CustomPoolState {
    pub tag: YieldPoolTag,
    pub yield_protocols: Vec<YieldProtocol>,
}

trait CustomPoolStateContainer {
    fn read_custom_state(&self) -> Result<CustomPoolState, ProgramError>;
    fn write_custom_state(&mut self, custom_state: &CustomPoolState) -> Result<(), ProgramError>;
}

impl CustomPoolStateContainer for PoolState {
    fn read_custom_state(&self) -> Result<CustomPoolState, ProgramError> {
        CustomPoolState::try_from_slice(&self.custom_state).map_err(|_| {
            msg!("Invalid pool custom state");
            ProgramError::InvalidAccountData
        })
    }

    fn write_custom_state(&mut self, custom_state: &CustomPoolState) -> Result<(), ProgramError> {
        self.custom_state = custom_state.try_to_vec().unwrap();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct YieldProtocol {
    pub name: String,
    pub yield_protocol_pubkey: Address,  // program id of lender
}

pub struct YieldPool;

impl Pool for YieldPool {
    fn initialize_pool(context: &PoolContext, state: &mut PoolState) -> Result<(), ProgramError> {
        if context.custom_accounts.len() < 1 {
            msg!("Missing yield protocol accounts.");
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        state.write_custom_state(&CustomPoolState::default())?;
        Ok(())
    }
}

#[cfg(not(feature = "no-entrypoint"))]
declare_pool_entrypoint!(YieldPool);
