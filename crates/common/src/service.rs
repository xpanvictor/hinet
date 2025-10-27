use tokio::sync::broadcast::Receiver;

pub trait Service {
    async fn run(
        self,
        shutdown_rx: Receiver<()>
    );
}
