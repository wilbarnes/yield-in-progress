use crate::{math::Rate, lending_protocols::LendingProtocol};

use std::convert::From;

use solana_program::{
    program_error::ProgramError,
    account_info::{AccountInfo, next_account_info},
    program_pack::{Pack}
};
use spl_token_lending::state::Reserve;

pub struct Oyster;

impl LendingProtocol for Oyster {
    fn calculate_deposit_APY(accounts: &[AccountInfo]) -> Result<Rate, ProgramError>{
        let accounts_iter = &mut accounts.iter();
        let reserve_account_info = next_account_info(accounts_iter)?;
        let reserve = Reserve::unpack(&reserve_account_info.data.borrow())?;
        let utilization_rate = Rate::from(reserve.state.current_utilization_rate());
        let borrow_rate = Rate::from(reserve.current_borrow_rate());
        let deposit_apy = utilization_rate * borrow_rate;
        Ok(deposit_apy)
    }
}
