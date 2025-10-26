use tokio::sync::broadcast::Receiver;

pub trait Service {
    async fn run(
        &mut self,
        shutdown_rx: Receiver<()>
    );
}
