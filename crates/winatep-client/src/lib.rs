//! Welocme to the Rust Doasif WINATEP client.
//!
//! TODO: Talk about the WINATEP project.
use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::{CloseCode, Message};
use snafu::prelude::*;
use winatep_wire_types::{InputMessage, OutputMessage, Screen};

fn display_websocket_err(source: &reqwest_websocket::Error) -> String {
    match source {
        reqwest_websocket::Error::Handshake(handshake_error) => handshake_error.to_string(),
        reqwest_websocket::Error::Reqwest(error) => error.to_string(),
        reqwest_websocket::Error::Tungstenite(error) => error.to_string(),
        e => format!("{e}"),
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}: {}", display_websocket_err(source)))]
    Websocket { source: reqwest_websocket::Error },

    #[snafu(display("{msg}"))]
    Driver { msg: String },

    #[snafu(display("Unexpected driver output message: {output_message:#?}"))]
    UnexpectedDriverMessage { output_message: OutputMessage },

    #[snafu(display("Driver closed the socket with {code}: {reason}"))]
    Closed { code: CloseCode, reason: String },

    #[snafu(display("Unexpected answer from the driver: {msg:#?}"))]
    UnexpectedAnswer { msg: Message },

    #[snafu(display("Unexpected end of websocket stream"))]
    End,

    #[snafu(display("Could not encode message: {source}"))]
    Encoding { source: serde_json::Error },

    #[snafu(display("Could not decode message: {source}"))]
    Decoding { source: serde_json::Error },
}

impl From<reqwest_websocket::Error> for Error {
    fn from(source: reqwest_websocket::Error) -> Self {
        Error::Websocket { source }
    }
}

/// Handles the error case of an output message.
macro_rules! handle_output_message {
    ($expression:expr, $pattern:pat => $result:ident) => {
        match $expression {
            $pattern => Ok($result),
            OutputMessage::Error(msg) => DriverSnafu { msg }.fail(),
            output_message => UnexpectedDriverMessageSnafu { output_message }.fail(),
        }
    };
}

/// Represents a connection to the WINATEP driver.
pub struct Session {
    socket: reqwest_websocket::WebSocket,
}

impl Session {
    /// Create a new session by connecting to a running instance of the driver.
    pub async fn new(url: impl reqwest::IntoUrl) -> Result<Self, Error> {
        let socket = reqwest_websocket::websocket(url).await?;
        Ok(Self { socket })
    }

    async fn recv(&mut self) -> Result<OutputMessage, Error> {
        match self.socket.next().await.context(EndSnafu)?? {
            reqwest_websocket::Message::Text(text) => {
                let msg: OutputMessage = serde_json::from_str(&text).context(DecodingSnafu)?;
                Ok(msg)
            }
            reqwest_websocket::Message::Binary(vec) => {
                let msg: OutputMessage = serde_json::from_slice(&vec).context(DecodingSnafu)?;
                Ok(msg)
            }
            reqwest_websocket::Message::Close { code, reason } => {
                log::warn!("socket closed by the driver: {code} {reason}");
                ClosedSnafu { code, reason }.fail()
            }
            msg => UnexpectedAnswerSnafu { msg }.fail(),
        }
    }

    async fn send(&mut self, msg: InputMessage) -> Result<OutputMessage, Error> {
        log::trace!("send: {msg:#?}");
        let text = serde_json::to_string(&msg).context(EncodingSnafu)?;
        self.socket.send(Message::Text(text)).await?;
        let rmsg = self.recv().await?;
        log::trace!("recv: {rmsg:#?}");
        Ok(rmsg)
    }

    pub async fn get_screens(&mut self) -> Result<Vec<Screen>, Error> {
        let msg = self.send(InputMessage::GetScreens).await?;
        handle_output_message!(msg, OutputMessage::GotScreens(screens) => screens)
    }
}
