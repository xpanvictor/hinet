use libp2p::{
    autonat, dcutr, gos, gossipsub, identify, kad, mdns, ping, relay, request_response,
    swarm::NetworkBehaviour,
};
use message::pb::chat_dm;

#[derive(NetworkBehaviour)]
pub struct MsgBehaviour {
    // discovery
    kad: kad::Behaviour<kad::store::MemoryStore>,
    mdns: mdns::tokio::Behaviour,
    identity: identify::Behaviour,

    // msg
    direct_message:
        request_response::cbor::Behaviour<chat_dm::DirectMessage, chat_dm::DirectMessageResponse>,
    group_chat: gossipsub::Behaviour,

    // utils
    identify: identify::Behaviour,
    ping: ping::Behaviour,

    // nat
    nat: autonat::Behaviour,
    dcutr: dcutr::Behaviour,
    relay_client: relay::Behaviour,
}

#[derive(Debug)]
pub enum AppEvent {
    Kad(kad::Event),
    Gossipsub(gossipsub::Event),
    DirectMessage(request_response::Event<chat_dm::DirectMessage, chat_dm::DirectMessageResponse>),
    GroupChat(gossipsub::Event),
    Identify(identify::Event),
    Ping(ping::Event),
    RelayClient(relay::Event),
    Dcutr(dcutr::Event),
}

impl From<kad::Event> for AppEvent {
    fn from(value: kad::Event) -> Self {
        AppEvent::Kad(value)
    }
}

impl From<gossipsub::Event> for AppEvent {
    fn from(value: gossipsub::Event) -> Self {
        AppEvent::Gossipsub(value)
    }
}

impl From<request_response::Event<chat_dm::DirectMessage, chat_dm::DirectMessageResponse>>
    for AppEvent
{
    fn from(
        value: request_response::Event<chat_dm::DirectMessage, chat_dm::DirectMessageResponse>,
    ) -> Self {
        AppEvent::DirectMessage(value)
    }
}
