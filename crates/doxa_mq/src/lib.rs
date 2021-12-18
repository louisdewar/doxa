use std::time::Duration;

use deadpool::managed::Timeouts;
pub use doxa_core::lapin;
pub use tokio_amqp;

pub use lapin::Connection;

use doxa_core::{deadpool_lapin::Manager, tokio, tracing::info};
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

/// TODO: this is more of stopgap because the competition system can't handle MQ not being on.
/// The better solution (compared to this) is to make the competition system auto reboot/retry on
/// connection issues.
///
/// This methods tries to get a MQ connection with a timeout of 2 seconds
pub async fn wait_for_mq(pool: &MQPool) {
    let mut i = 0;

    loop {
        match pool
            .timeout_get(&Timeouts {
                wait: Some(Duration::from_secs(2)),
                create: Some(Duration::from_secs(2)),
                recycle: Some(Duration::from_secs(2)),
            })
            .await
        {
            Ok(_) => return,
            Err(e) => {
                i += 1;
                // Only four attempts
                if i == 4 {
                    panic!("failed to connect to rabbit mq: {}", e);
                }

                info!(attempt=%i, "failed to connect to rabbit mq, trying again");

                tokio::time::sleep(Duration::from_millis(750)).await;
            }
        }
    }
}
