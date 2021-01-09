use serum_pool::schema::declare_tag;

use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};

declare_tag!(YieldInstructionTag, u64, 0x31e6452361a17878);

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct YieldInstruction {
    tag: YieldInstructionTag,
    inner: YieldInstructionInner,
}

// NOTE: how to dynamically add new lending pools?

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub enum YieldInstructionInner {
    /// Optimise between lenders
    ///
    /// Accounts:
    ///
        Optimise
}
