use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::{join, select, spawn};
use tokio::sync::broadcast::Receiver;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use futures::{SinkExt, StreamExt};
use common::MsgBus;
use common::service::Service;
use bytes::Bytes;

pub struct Network{
    msg_bus: Arc<MsgBus>,
}

pub struct MsgReceived {
    pub msg: Vec<u8>,
}

#[derive(Clone)]
pub struct NetSendMsg {
    pub msg: Bytes,
    pub addr: SocketAddr,
}

impl Network {
    pub fn new(msg_bus: Arc<MsgBus>) -> Network {
        Network{
            msg_bus
        }
    }

    async fn listen(this: Arc<Self>, mut shutdown: Receiver<()>, address: impl ToSocketAddrs) {
        let listener = TcpListener::bind(address).await.unwrap();
        loop {
            let bus = this.msg_bus.clone();

            select! {
                _ = shutdown.recv() => {tracing::info!("received shutdown signal")},
                accept_rs = listener.accept() => {
                    let (socket, addr) = match accept_rs {
                        Ok((stream, addr)) => {
                            tracing::info!("new connection from {}", addr);
                            (stream, addr)
                        }
                        Err(err) => {
                            tracing::warn!("error accepting new connection: {}", err);
                            continue;
                        }
                    };
                    spawn(async move {
                        if let Err(e) = Network::handle_conn(socket, addr, bus.clone()).await {
                            tracing::warn!("error handling connection from {}: {}", addr, e);
                        };
                    });
                }
            }
        }
    }

    async fn handle_conn(socket: TcpStream, addr: SocketAddr, bus: Arc<MsgBus>) -> anyhow::Result<()> {
        // use length delimited codec
        let mut framed = Framed::new(socket, LengthDelimitedCodec::new());
        while let Some(frame) = framed.next().await {
            match frame {
                Ok(bytes) => {
                    let msg = bytes.to_vec();
                    tracing::info!("received {} bytes from {}", msg.len(), addr);
                    bus.publish(MsgReceived{ msg }).await;
                }
                Err(e) => {
                    tracing::warn!("error reading from socket: {}", e);
                    return Err(anyhow::anyhow!("error reading from socket: {}", e));
                }
            }
        }
        Ok(())
    }

    async fn handle_msg(this: Arc<Self>, bus: Arc<MsgBus>, mut shutdown: Receiver<()>) -> anyhow::Result<()> {
        let mut rx = bus.subscribe::<NetSendMsg>().await;
        select! {
            _ = shutdown.recv() => {
                tracing::info!("stopping outgoing msgs");
                Ok(())
            },
            msg = rx.recv() => {
                // spawn a new thread to handle this
                spawn(async move {
                    if let Ok(msg) = msg {
                        if let Ok(socket) = TcpStream::connect(msg.addr).await {
                            let mut framed = Framed::new(socket, LengthDelimitedCodec::new());
                            if let Err(e) = framed.send(msg.msg).await {
                                tracing::warn!("error sending message: {}", e);
                            }
                        };
                    }
                });
                Ok(())
            }
        }
    }
}

impl Service for Network {
    async fn run(mut self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let srx_listener = shutdown_rx.resubscribe();
        let listener_handle = tokio::spawn(Network::listen(
            Arc::new(self),
            srx_listener,
            "localhost:5052"
        ));
        select! {
            _ = shutdown_rx.recv() => {
                tracing::info!("network shutdown");
            }
        }
        let _ = join!(listener_handle);
    }
}
