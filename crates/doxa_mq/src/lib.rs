pub use doxa_core::lapin;
pub use tokio_amqp;

pub use lapin::Connection;

use doxa_core::deadpool_lapin::Manager;
use lapin::ConnectionProperties;
use tokio_amqp::LapinTokioExt;

pub use doxa_core::deadpool_lapin::Pool as MQPool;

pub mod action;
pub mod model;

/// Get a dedicated connection to the message queue
pub async fn establish_mq_connection(addr: &str) -> Result<Connection, lapin::Error> {
    Connection::connect(addr, ConnectionProperties::default().with_tokio()).await
}

pub async fn establish_pool(addr: String, max_connections: usize) -> MQPool {
    let manager = Manager::new(addr, ConnectionProperties::default().with_tokio());
    MQPool::new(manager, max_connections)
}
