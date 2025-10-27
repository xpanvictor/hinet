use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use common::MsgBus;
use common::service::Service;

pub struct Network{
    msg_bus: Arc<MsgBus>,
}

impl Network {
    pub fn new(msg_bus: Arc<MsgBus>) -> Network {
        Network{
            msg_bus
        }
    }

    async fn serve(&mut self) {
        self.msg_bus.publish("hello world").await;
    }
}

impl Service for Network {
    async fn run(mut self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        select! {
            _ = shutdown_rx.recv() => {
                tracing::info!("network shutdown");
            }
            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                self.serve().await;
            }
        }
    }
}
