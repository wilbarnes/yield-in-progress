use solana_program::{
    program_error::ProgramError,
    account_info::AccountInfo
};

use crate::math::Rate;

pub trait LendingProtocol {
    #[allow(non_snake_case)]
    fn calculate_deposit_APY(accounts: &[AccountInfo]) -> Result<Rate, ProgramError>;
}
