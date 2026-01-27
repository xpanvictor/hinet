use std::{future, process::Output};

use tokio::sync::broadcast::Receiver;

pub trait Service {
    fn run(self, shutdown_rx: Receiver<()>) -> impl future::Future<Output = ()> + Send;
}
