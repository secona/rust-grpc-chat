use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use proto::chat_server::{Chat, ChatServer};
use proto::{ChatMessage, ConnectRequest};
use tokio::sync::{mpsc, RwLock};
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("chat");
}

#[derive(Debug, Default)]
struct Hub {
    pub senders: HashMap<String, mpsc::Sender<ChatMessage>>,
}

impl Hub {
    async fn broadcast(&self, msg: ChatMessage) {
        for (_, tx) in &self.senders {
            let _ = tx.send(msg.clone()).await;
        }
    }
}

#[derive(Debug, Default)]
pub struct ChatService {
    hub: Arc<RwLock<Hub>>,
}

impl ChatService {
    pub fn new() -> ChatServer<Self> {
        let hub = Arc::new(RwLock::new(Hub::default()));
        ChatServer::new(Self { hub })
    }
}

#[tonic::async_trait]
impl Chat for ChatService {
    type ConnectServerStream = Pin<
        Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send + Sync>,
    >;

    async fn connect_server(&self, request: Request<ConnectRequest>) -> Result<Response<Self::ConnectServerStream>, Status> {
        let message = request.get_ref();
        let username = message.username.clone();

        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx) = mpsc::channel(1);

        self.hub.write().await.senders.insert(username.clone(), tx);

        let hub_clone = self.hub.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        hub_clone.write().await.senders.remove(&username);
                    }
                }
            }
        });

        println!("senders count = {}", self.hub.read().await.senders.len());

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))
    }

    async fn send(&self, request: Request<ChatMessage>) -> Result<Response<ChatMessage>, Status> {
        let message = request.into_inner().message;
        let chat_msg = ChatMessage { message };

        let hub = self.hub.read().await;
        hub.broadcast(chat_msg.clone()).await;

        Ok(Response::new(chat_msg))
    }
}
