//! Welocme to the Rust Doasif WINATEP client.
//!
//! TODO: Talk about the WINATEP project.
use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::{CloseCode, Message};
use snafu::prelude::*;
use winatep_wire_types::{
    BoundingRectangle, FindImageFilter, FindImageQuality, InputMessage, OutputMessage, Screen,
    Token, Vec2,
};

pub use winatep_wire_types::ImageBuffer;

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
macro_rules! txrx {
    ($self:ident, $input:expr, $output:pat => $result:expr) => {{
        let msg = $self.send($input).await?;
        match msg {
            $output => Ok($result),
            OutputMessage::Error(e) => DriverSnafu { msg: e }.fail(),
            output_message => UnexpectedDriverMessageSnafu { output_message }.fail(),
        }
    }};
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
        txrx!(self, InputMessage::GetScreens, OutputMessage::GotScreens(screens) => screens)
    }

    pub async fn get_main_screen(&mut self) -> Result<Screen, Error> {
        txrx!(self, InputMessage::GetMainScreen, OutputMessage::GotMainScreen(screen) => screen)
    }

    pub async fn capture_screen(
        &mut self,
        screen_name: impl AsRef<str>,
    ) -> Result<ImageBuffer, Error> {
        txrx!(
            self,
            InputMessage::CaptureScreen {
                name: screen_name.as_ref().to_string(),
            },
            OutputMessage::CapturedScreen { image_buffer } => image_buffer
        )
    }

    pub async fn get_mouse_location(&mut self) -> Result<Vec2, Error> {
        txrx!(self, InputMessage::GetMouseLocation, OutputMessage::GotMouseLocation(loc) => loc)
    }

    pub async fn input(&mut self, token: Token) -> Result<(), Error> {
        txrx!(self, InputMessage::DoInput(token), OutputMessage::DidInput => ())
    }

    pub async fn text(&mut self, text: impl AsRef<str>) -> Result<(), Error> {
        txrx!(self, InputMessage::DoTypeText(text.as_ref().to_owned()), OutputMessage::DidTypeText => ())
    }

    pub async fn find_text_in_screen(
        &mut self,
        screen_name: impl AsRef<str>,
        text: impl AsRef<str>,
        timeout_in_seconds: f32,
    ) -> Result<Vec<BoundingRectangle>, Error> {
        txrx!(
            self,
            InputMessage::FindText {
                text: text.as_ref().to_owned(),
                screen_name: screen_name.as_ref().to_owned(),
                timeout_in_seconds
            },
            OutputMessage::FoundText { locations } => locations
        )
    }

    pub async fn find_image_in_screen(
        &mut self,
        screen_name: impl AsRef<str>,
        image: ImageBuffer,
        quality: FindImageQuality,
        filter: FindImageFilter,
    ) -> Result<Vec<BoundingRectangle>, Error> {
        txrx!(
            self,
            InputMessage::FindImage {
                screen_name: screen_name.as_ref().to_owned(),
                image,
                quality,
                filter
            },
            OutputMessage::FoundText { locations } => locations
        )
    }
}
