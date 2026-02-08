use std::{collections::HashSet, sync::Arc};

use common::MsgBus;
use libp2p::{
    PeerId, Swarm, identify, kad, mdns,
    swarm::{SwarmEvent, behaviour::ConnectionEstablished},
};
use tracing::warn;

use crate::behavior::{AppEvent, MsgBehaviour};

pub struct P2pSwarmHandler {
    connected_peers: HashSet<PeerId>,
    msg_bus: Arc<MsgBus>,
    swarm: &mut Swarm<MsgBehaviour>,
}

impl P2pSwarmHandler {
    pub fn new(swarm: &mut Swarm<MsgBehaviour>, msg_bus: Arc<MsgBus>) -> Self {
        Self {
            connected_peers: HashSet::new(),
            msg_bus,
            swarm,
        }
    }

    pub fn handle_event(&mut self, event: SwarmEvent<AppEvent>) {
        match event {
            SwarmEvent::NewListenAddr {
                listener_id,
                address,
            } => {
                tracing::info!("Local node is listening on {}", address);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                tracing::debug!("Connected to new node: {}", peer_id);
                self.connected_peers.insert(peer_id);
            }
            // handling kad ops
            SwarmEvent::Behaviour(AppEvent::Kad(kad_event)) => match kad_event {
                kad::Event::OutboundQueryProgressed { result, .. } => match result {
                    kad::QueryResult::GetClosestPeers(Ok(ok)) => {
                        if ok.peers.is_empty() {
                            warn!("swarm/kad", "no closest peers found");
                        }
                        tracing::info!("peers {}", ok.peers)
                    }
                    _ => {
                        tracing::warn!("swarm/kad", "other results kad")
                    }
                },
                ev => {
                    tracing::warn!("swarm/kad", "event not handled: {}", ev);
                }
            },
            SwarmEvent::Behaviour(AppEvent::Mdns(ev)) => match ev {
                mdns::Event::Discovered(peers) => {
                    for (peer, addr) in peers {
                        // add to kad n dial
                        self.swarm
                            .behaviour_mut()
                            .kad
                            .add_address(&peer, addr.clone());
                        self.swarm.dial(addr)?
                    }
                }
                mdns::Event::Expired(peers) => {}
            },
            // identify
            SwarmEvent::Behaviour(AppEvent::Identify(identify_ev)) => match identify_ev {
                identify::Event::Received {
                    connection_id,
                    peer_id,
                    info,
                } => {
                    // add to kad
                    for addr in info.listen_addrs {
                        self.swarm.behaviour_mut().kad.add_address(&peer_id, addr);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
