use std::{
    default,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
    time::Duration,
};

use common::{
    MsgBus,
    events::{NetworkEvents, SystemEvents},
    service::Service,
};
use futures::StreamExt;
use libp2p::{
    PeerId, StreamProtocol, Swarm, SwarmBuilder, Transport, autonat,
    core::upgrade::Version,
    dcutr, gossipsub, identify,
    identity::Keypair,
    kad,
    mdns::{self},
    noise, ping,
    relay::{self},
    request_response, swarm, tcp, yamux,
};
use message::pb::chat_dm::{DirectMessage, Text, direct_message};

use crate::{
    behavior::{self, DmProtobufCodec, MsgBehaviour},
    handler::P2pSwarmHandler,
    tcp::NetSendMsg,
};

pub struct P2PSwarm {
    msg_bus: Arc<MsgBus>,
    swarm: Swarm<MsgBehaviour>,
    handler: P2pSwarmHandler,
}

impl P2PSwarm {
    pub fn new(identity: Keypair, msg_bus: Arc<MsgBus>) -> Self {
        let mut swarm = P2PSwarm::build_swarm(identity).unwrap();
        P2PSwarm {
            msg_bus,
            swarm,
            handler: P2pSwarmHandler::new(&mut swarm, msg_bus),
        }
    }

    fn build_swarm(kp: Keypair) -> anyhow::Result<Swarm<MsgBehaviour>> {
        let peer_id = PeerId::from_public_key(&kp.public());

        let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default());
        let mux = yamux::Config::default();
        let noise = noise::Config::new(&kp).expect("noise error");
        let (relay_transport, relay_client) = relay::client::new(peer_id);
        let transport = tcp_transport
            .or_transport(relay_transport)
            .upgrade(Version::V1)
            .authenticate(noise)
            .multiplex(mux);

        let identify = identify::Behaviour::new(identify::Config::new(
            "/messaging/1.0.0".into(), // todo: standard protocol versions
            kp.public(),
        ));
        let kad = kad::Behaviour::new(peer_id, kad::store::MemoryStore::new(peer_id));
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)
            .expect("couldn't create mdns node");

        let dm_protocol = std::iter::once((
            StreamProtocol::new("/messaging/dm/1.0.0"),
            request_response::ProtocolSupport::Full,
        ));
        let direct_message =
            request_response::Behaviour::new(dm_protocol, request_response::Config::default());
        let gc_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(gc_fn)
            .build()
            .expect("cant sub config");
        let group_chat =
            gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(kp), gossipsub_config)
                .expect("err gc");

        let ping = ping::Behaviour::new(ping::Config::default());
        let nat = autonat::Behaviour::new(peer_id, autonat::Config::default());
        let dcutr = dcutr::Behaviour::new(peer_id);

        let behavior = MsgBehaviour {
            identify,
            direct_message,
            group_chat,
            kad,
            mdns,
            ping,
            nat,
            dcutr,
            relay_client: relay::Behaviour::new(peer_id, relay::Config::default()),
        };
        SwarmBuilder::with_existing_identity(kp)
            .with_tokio()
            .with_other_transport(transport)
            .with_behaviour(behavior)?
    }

    pub fn listen(&self) -> Result<()> {
        // todo: listen other layers
        self.swarm.listen_on("ip4/0.0.0.0/tcp/0".parse()?);
        Ok(())
    }
}

impl Service for P2PSwarm {
    async fn run(self, mut shutdown: tokio::sync::broadcast::Receiver<()>) {
        tracing::info!("P2P Swarm service started");
        // subscribe to network events
        tracing.info(bus_event = "NetworkSend");
        let bus_rx = self.msg_bus.subscribe::<NetworkEvents>().await;

        self.swarm
            .behaviour_mut()
            .kad
            .set_mode(Some(kad::Mode::Server));
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    tracing::info!("P2P Swarm service shutting down");
                    break;
                },
                // handle swarm events
                event = self.swarm.select_next_some() => self.handler.handle_event(event),
                // handle internal commands (from bus)
                command = bus_rx.recv()? => {
                    println!("recv {:?}", command);
                    if matches!(command, NetworkEvents::SendMsg) {
                        self.swarm.behaviour_mut().direct_message.send_request(todo!(), DirectMessage {
                            id: "x1".into(),
                            timestamp: 2,
                            payload: Some(Text("hello world"))
                        });
                    }
                }
            }
        }
        tracing::info!("P2P Swarm service stopped");
    }
}
