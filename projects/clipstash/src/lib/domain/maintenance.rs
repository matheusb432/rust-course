use crate::data::DatabasePool;
use crate::service;
use std::time::Duration;
use tokio::runtime::Handle;

pub struct Maintenance;

impl Maintenance {
    /// Deletes expired clips every 10 seconds
    pub fn spawn(pool: DatabasePool, handle: Handle) -> Self {
        // NOTE spawn will immediately spawn this async task
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                if let Err(e) = service::action::delete_expired(&pool).await {
                    eprintln!("failed to delete expired clips: {e}");
                }
            }
        });
        Self
    }
}
