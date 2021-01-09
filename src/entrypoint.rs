use serum_pool::declare_pool_entrypoint;
use crate::yield_pool::YieldPool;

#[cfg(not(feature = "no-entrypoint"))]
declare_pool_entrypoint!(YieldPool);
