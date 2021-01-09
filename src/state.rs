use serum_pool::{schema::{declare_tag, Address, PoolState}};
use solana_program::{
    msg,
    program_error::ProgramError};

use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};

// NOTE: why do we need this?
declare_tag!(YieldPoolTag, u64, 0x4a3ab7f76f93f94e);

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Default)]
pub struct CustomPoolState {
    pub tag: YieldPoolTag,
    pub name: String,
    pub asset_pubkey: Address,
    pub lending_protocols: Vec<LendingProtocolState>,
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LendingProtocolState {
    pub name: String,
    pub lending_protocol_program_id: Address,
    pub num_accounts: u8
}

pub trait CustomPoolStateContainer {
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
