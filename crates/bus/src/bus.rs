use std::any::{Any, TypeId};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

/// MsgBus: should allow services register for certain events
/// Publishes those events to all subscribers
/// Minimizes and centralizes events for now
pub struct MsgBus {
    subscribers: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl MsgBus {
    pub fn new() -> Self {
        MsgBus {
            subscribers: HashMap::new(),
        }
    }

    pub fn register<T: Send + 'static + std::clone::Clone>(&mut self) -> Receiver<T> {
        let type_id = TypeId::of::<T>();

        if !self.subscribers.contains_key(&type_id) {
            let (tx, _rx) = broadcast::channel::<T>(1024); // todo: use settings conf
            self.subscribers.insert(type_id, Box::new(tx));
        }

        let sender = self
            .subscribers
            .get(&type_id)
            .unwrap()
            .downcast_ref::<Sender<T>>()
            .expect("type mismatch in bus");
        sender.subscribe()
    }

    pub fn send<T: Any + Send>(&mut self, msg: T) {
        let type_id = TypeId::of::<T>();
        if let Some(tx) = self.subscribers.get(&type_id) {
            let sender = tx.downcast_ref::<Sender<T>>().unwrap();
            let _ = sender.send(msg);
        } else {
            tracing::warn!("bus::send: no subscriber to type {:?}", type_id);
        }
    }
}
