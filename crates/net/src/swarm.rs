use std::sync::Arc;

use common::{MsgBus, service::Service};
use libp2p::{Swarm, core::upgrade::Version, identity::Keypair, noise, tcp, yamux};

use crate::behavior::MsgBehaviour;

pub struct P2PSwarm {
    msg_bus: Arc<MsgBus>,
    swarm: Swarm<MsgBehaviour>,
}

impl P2PSwarm {
    pub fn new(identity: Keypair, msg_bus: Arc<MsgBus>) -> Self {
        P2PSwarm {
            msg_bus,
            swarm: P2PSwarm::build_swarm(identity),
        }
    }

    fn build_swarm(identity: Keypair) -> Swarm<MsgBehaviour> {
        let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default());
        let mux = yamux::Config::default();
        let noise = noise::Config::new(&identity);
        let transport = tcp_transport
            .upgrade(Version::V1)
            .authenticate(noise)
            .multiplex(mux);

        sw
    }
}

impl Service for P2PSwarm {
    async fn run(self, mut shutdown: tokio::sync::broadcast::Receiver<()>) {
        tracing::info!("P2P Swarm service started");
        // Placeholder for swarm logic
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    tracing::info!("P2P Swarm service shutting down");
                    break;
                }
                // Add swarm operations here
            }
        }
        tracing::info!("P2P Swarm service stopped");
    }
}
