use std::any::{Any, TypeId};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{RwLock, broadcast};

/// MsgBus: should allow services register for certain events
/// Publishes those events to all subscribers
/// Minimizes and centralizes events for now
pub struct MsgBus {
    inner_bus: InnerBus,
}

struct InnerBus {
    subscribers: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}
impl MsgBus {
    pub fn new() -> Self {
        MsgBus {
            inner_bus: InnerBus {
                subscribers: RwLock::new(HashMap::new()),
            },
        }
    }

    pub async fn subscribe<T: Send + Clone + Sync + 'static>(&self) -> Receiver<T> {
        let type_id = TypeId::of::<T>();

        let mut inner = self.inner_bus.subscribers.write().await;
        if !inner.contains_key(&type_id) {
            let (tx, _rx) = broadcast::channel::<T>(1024); // todo: use settings conf
            inner.insert(type_id, Box::new(tx));
        }

        let sender = inner
            .get(&type_id)
            .unwrap()
            .downcast_ref::<Sender<T>>()
            .expect("type mismatch in bus");
        sender.subscribe()
    }

    pub async fn publish<T: Any + Send>(&self, msg: T) {
        let type_id = TypeId::of::<T>();
        let inner = self.inner_bus.subscribers.read().await;
        if let Some(tx) = inner.get(&type_id) {
            let sender = tx.downcast_ref::<Sender<T>>().unwrap();
            let _ = sender.send(msg);
        } else {
            tracing::warn!("bus::send: no subscriber to type {:?}", type_id);
        }
    }
}
