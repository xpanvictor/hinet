use futures::AsyncReadExt;
use futures::AsyncWriteExt;
use futures::io;
use futures::io::{AsyncRead, AsyncWrite};
use libp2p::{StreamProtocol, request_response::Codec};
use message::Message;
use message::pb::chat_dm::{self, DirectMessage, DirectMessageResponse};

#[derive(Clone, Default)]
pub struct DmProtobufCodec;

#[async_trait::async_trait]
impl Codec for DmProtobufCodec {
    #[doc = " The type of protocol(s) or protocol versions being negotiated."]
    type Protocol = StreamProtocol;

    #[doc = " The type of inbound and outbound requests."]
    type Request = DirectMessage;

    #[doc = " The type of inbound and outbound responses."]
    type Response = DirectMessageResponse;

    async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;

        chat_dm::DirectMessage::decode(&*buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;

        chat_dm::DirectMessageResponse::decode(&*buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let mut buf = Vec::new();
        req.encode(&mut buf).unwrap();
        io.write_all(&buf).await
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let mut buf = Vec::new();
        res.encode(&mut buf).unwrap();
        io.write_all(&buf).await
    }
}
