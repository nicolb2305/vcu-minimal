use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::api::ws_types::SubscriptionType;
use crate::utils::port_and_auth;

pub struct WSClient {
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

type Error = Box<dyn std::error::Error>;

#[derive(PartialEq)]
pub enum Events {
    Json,
    Lcds,
    Callback,
    None,
}

impl WSClient {
    pub async fn connect(events: Events) -> Result<Self, Error> {
        let (port, auth) = port_and_auth()?;

        let cert = native_tls::Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
        let tls = native_tls::TlsConnector::builder()
            .add_root_certificate(cert)
            .build()?;
        let connector = tokio_tungstenite::Connector::NativeTls(tls);
        let mut url = format!("wss://127.0.0.1:{port}").into_client_request()?;
        {
            let headers = url.headers_mut();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(format!("Basic {auth}").as_str())?,
            );
        }

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, Some(connector)).await?;

        let (mut write, read) = ws_stream.split();

        match events {
            Events::Json => write.send(Message::text("[5, \"OnJsonApiEvent\"]")).await?,
            Events::Lcds => write.send(Message::text("[5, \"OnLcdsEvent\"]")).await?,
            Events::Callback => write.send(Message::text("[5, \"OnCallback\"]")).await?,
            Events::None => (),
        }

        Ok(Self { write, read })
    }

    pub async fn subscribe(&mut self, event: String) -> Result<(), Error> {
        self.write
            .send(Message::text(format!("[5, {event}]")))
            .await?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, event: String) -> Result<(), Error> {
        self.write
            .send(Message::text(format!("[6, {event}]")))
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct WebSocketClient {
    pub socket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl WebSocketClient {
    pub async fn new() -> Result<Self, Error> {
        let (port, auth) = port_and_auth()?;

        let cert = native_tls::Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
        let tls = native_tls::TlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .unwrap();
        let connector = tokio_tungstenite::Connector::NativeTls(tls);

        let mut url = format!("wss://127.0.0.1:{port}")
            .into_client_request()
            .unwrap();
        {
            let headers = url.headers_mut();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(format!("Basic {auth}").as_str())?,
            );
        }

        let (socket, _) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, Some(connector))
                .await
                .unwrap();
        Ok(Self { socket })
    }

    pub async fn subscribe(&mut self, event: SubscriptionType) -> Result<(), Error> {
        self.socket
            .send(Message::text(format!(r#"[5, "{event}"]"#)))
            .await?;

        Ok(())
    }
}
