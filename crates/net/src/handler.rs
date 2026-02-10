use std::{collections::HashSet, sync::Arc};

use common::MsgBus;
use libp2p::{
    PeerId, Swarm, gossipsub, identify, kad, mdns,
    swarm::{SwarmEvent, behaviour::ConnectionEstablished},
};
use tracing::warn;

use crate::behavior::{AppEvent, MsgBehaviour};

pub struct P2pSwarmHandler {
    connected_peers: HashSet<PeerId>,
    msg_bus: Arc<MsgBus>,
    swarm: &mut Swarm<MsgBehaviour>,
}

// todo: use dial queue to avoid storm dialing
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
                        tracing::info!("peers {}", ok.peers);
                        // can confirm if target peer but just redial if not conn
                        for peer_info in ok.peers {
                            if self.connected_peers.contains(&peer_info.peer_id) {
                                continue;
                            }
                            tracing::info!("Dialing discovered peer: {}", peer_info);
                            if let Err(err) = self.swarm.dial(peer_info) {
                                tracing::warn!("swarm/kad", "Dial failed for {}", peer_info)
                            }
                        }
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
                mdns::Event::Expired(peers) => {
                    for (peer, _) in peers {
                        self.connected_peers.remove(&peer);
                    }
                }
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
                        // can store peer_info here
                    }
                }
                _ => {}
            },
            SwarmEvent::Behaviour(AppEvent::Gossipsub(gsub_ev)) => match gsub_ev {
                gossipsub::Event::Message {
                    propagation_source,
                    message_id,
                    message,
                } => {
                    tracing::info!("swarm/gsub", "message received {}", message)
                    // here process message, match to appropriate handler
                }
                _ => tracing::debug!("swarm/gsub", "unsupported event {}", gsub_ev),
            },
            _ => {}
        }
    }
}
