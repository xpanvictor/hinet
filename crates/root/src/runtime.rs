use std::sync::{Arc, RwLock};
use tokio;
use anyhow;
use tokio::select;
use tracing::instrument;
use metrics::counter;
use crate::metrics::Metrics;
use common::{MsgBus};
use common::service::Service;
use net::TcpNetwork;

pub struct Runtime;

impl Runtime {
    #[instrument]
    pub async fn run() -> anyhow::Result<()> {
        // basic tracing conf
        common::tracing::init_tracing()?;
        let msg_bus = Arc::new(MsgBus::new());
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

        // instantiate services
        let net = TcpNetwork::new(msg_bus.clone());

        // spawn services
        let net_handle = tokio::spawn(net.run(shutdown_tx.subscribe()));

        let system_task_counter = counter!(Metrics::R_TST, "system" => "system");
        system_task_counter.increment(1);

        // tmp data; todo: clean up
        let mut rx = msg_bus.subscribe::<&str>().await;
        select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::debug!("system received SIGINT");
            },
            resp = rx.recv() => {tracing::info!("got msg: {:?}", resp)}
            _ = shutdown_rx.recv() => {tracing::debug!("received shutdown signal")},
        }
        let _ = shutdown_tx.send(());
        let _ = tokio::join!(
            net_handle
        );
        Ok(())
    }
}
