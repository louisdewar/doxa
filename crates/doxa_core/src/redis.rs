//! Support for creating redis connection pools.
//!
//! Similar systems exist in `doxa_db` and `doxa_mq` but this is simplistic enough that it can go
//! here, in `doxa_core`.

pub use deadpool_redis::Pool as RedisPool;
pub use deadpool_redis::PoolError as RedisPoolError;
pub use redis::RedisError;

pub use redis;

use deadpool_redis::Manager;
use redis::IntoConnectionInfo;

pub async fn establish_pool<C: IntoConnectionInfo>(addr: C, max_connections: usize) -> RedisPool {
    let manager = Manager::new(addr).expect("creating redis client failed");
    RedisPool::builder(manager)
        .max_size(max_connections)
        .build()
        .expect("failed to create redis pool")
}
