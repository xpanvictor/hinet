use tokio;
use anyhow;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("received ctrl+c, shutting down");
        }
        _ = shutdown_rx.recv() => {
            tracing::info!("received shutdown signal");
        }
    }

    let _ = shutdown_tx.send(());

    Ok(())
}
