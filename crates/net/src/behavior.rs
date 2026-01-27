mod dm_msg;
use libp2p::{
    autonat, dcutr, gossipsub, identify, kad, mdns, ping, relay, request_response,
    swarm::NetworkBehaviour,
};

pub use crate::behavior::dm_msg::DmProtobufCodec;
use message::pb::chat_dm;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "AppEvent")]
pub struct MsgBehaviour {
    // discovery
    pub kad: kad::Behaviour<kad::store::MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub identify: identify::Behaviour,

    // msg
    pub direct_message: request_response::Behaviour<DmProtobufCodec>,
    pub(crate) group_chat: gossipsub::Behaviour,

    // utils
    pub(crate) ping: ping::Behaviour,

    // nat
    pub(crate) nat: autonat::Behaviour,
    pub(crate) dcutr: dcutr::Behaviour,
    pub(crate) relay_client: relay::Behaviour,
}

#[derive(Debug)]
pub enum AppEvent {
    Kad(kad::Event),
    Gossipsub(gossipsub::Event),
    DirectMessage(request_response::Event<chat_dm::DirectMessage, chat_dm::DirectMessageResponse>),
    GroupChat(gossipsub::Event),
    Identify(identify::Event),
    Ping(ping::Event),
    Mdns(mdns::Event),
    RelayClient(relay::Event),
    Dcutr(dcutr::Event),
    AutoNat(autonat::Event),
}

impl From<kad::Event> for AppEvent {
    fn from(value: kad::Event) -> Self {
        AppEvent::Kad(value)
    }
}

impl From<relay::Event> for AppEvent {
    fn from(value: relay::Event) -> Self {
        AppEvent::RelayClient(value)
    }
}

impl From<mdns::Event> for AppEvent {
    fn from(value: mdns::Event) -> Self {
        AppEvent::Mdns(value)
    }
}

impl From<dcutr::Event> for AppEvent {
    fn from(value: dcutr::Event) -> Self {
        AppEvent::Dcutr(value)
    }
}

impl From<gossipsub::Event> for AppEvent {
    fn from(value: gossipsub::Event) -> Self {
        AppEvent::Gossipsub(value)
    }
}

impl From<identify::Event> for AppEvent {
    fn from(value: identify::Event) -> Self {
        AppEvent::Identify(value)
    }
}

impl From<autonat::Event> for AppEvent {
    fn from(value: autonat::Event) -> Self {
        AppEvent::AutoNat(value)
    }
}

impl From<ping::Event> for AppEvent {
    fn from(value: ping::Event) -> Self {
        AppEvent::Ping(value)
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
