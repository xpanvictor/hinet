use tokio;
use anyhow;
use tokio::select;
use tracing::instrument;
use metrics::counter;
use crate::metrics::Metrics;

pub struct Runtime;

impl Runtime {
    #[instrument]
    pub async fn run() -> anyhow::Result<()> {
        // basic tracing conf
        common::tracing::init_tracing()?;

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);
        let system_task_counter = counter!(Metrics::R_TST, "system" => "system");
        system_task_counter.increment(1);
        select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("received SIGINT");
            },
            _ = shutdown_rx.recv() => {tracing::info!("received shutdown signal")},
        }
        let _ = shutdown_tx.send(());
        let _ = tokio::join!();
        Ok(())
    }
}
