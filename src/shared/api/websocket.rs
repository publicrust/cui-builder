use futures::{SinkExt, StreamExt};
#[cfg(not(target_arch = "wasm32"))]
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    WebSocketStream,
    MaybeTlsStream,
};
#[cfg(not(target_arch = "wasm32"))]
use tokio::net::TcpStream;
#[cfg(target_arch = "wasm32")]
use gloo_net::websocket::futures::WebSocket;
#[cfg(target_arch = "wasm32")]
use gloo_net::websocket::Message as WsMessage;
use url::Url;

pub struct WebSocketClient {
    #[cfg(not(target_arch = "wasm32"))]
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    #[cfg(target_arch = "wasm32")]
    socket: Option<WebSocket>,
}

impl WebSocketClient {
    pub fn new() -> Self {
        Self { 
            #[cfg(not(target_arch = "wasm32"))]
            stream: None,
            #[cfg(target_arch = "wasm32")]
            socket: None,
        }
    }

    pub async fn connect(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(url)?;

        #[cfg(target_arch = "wasm32")]
        {
            self.socket = Some(WebSocket::open(url.as_str())?);
            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (ws_stream, _) = connect_async(url).await?;
            self.stream = Some(ws_stream);
            Ok(())
        }
    }

    pub async fn send_message(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(socket) = &mut self.socket {
                socket.send(WsMessage::Text(message.to_string())).await?;
                Ok(())
            } else {
                Err("WebSocket not connected".into())
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(stream) = &mut self.stream {
                stream.send(Message::Text(message.to_string())).await?;
                Ok(())
            } else {
                Err("WebSocket not connected".into())
            }
        }
    }

    pub async fn receive_message(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(socket) = &mut self.socket {
                if let Some(msg) = socket.next().await {
                    match msg? {
                        WsMessage::Text(text) => Ok(text),
                        _ => Err("Received non-text message".into()),
                    }
                } else {
                    Err("No message received".into())
                }
            } else {
                Err("WebSocket not connected".into())
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(stream) = &mut self.stream {
                if let Some(msg) = stream.next().await {
                    match msg? {
                        Message::Text(text) => Ok(text),
                        _ => Err("Received non-text message".into()),
                    }
                } else {
                    Err("No message received".into())
                }
            } else {
                Err("WebSocket not connected".into())
            }
        }
    }
} 