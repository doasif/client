//! Welocme to the Rust Doasif WINATEP client.
//!
//! TODO: Talk about the WINATEP project.
use async_net::TcpStream;
use async_tungstenite::{
    async_tls::ClientStream,
    tungstenite::{
        client::IntoClientRequest,
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message, Utf8Bytes,
    },
};
use futures_util::StreamExt;
use snafu::{prelude::*, ResultExt};

pub use glam::Vec2;
use tracing::Instrument;
pub use winatep_wire_types::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Websocket {
        source: async_tungstenite::tungstenite::Error,
    },

    #[snafu(display("URI is missing the host"))]
    MissingHost,

    #[snafu(display("URI is missing the port"))]
    MissingPort,

    #[snafu(display("{msg}"))]
    Driver { msg: String },

    #[snafu(display("Unexpected driver output message: {output_message:#?}"))]
    UnexpectedDriverMessage { output_message: OutputMessage },

    #[snafu(display("Driver closed the socket with {code}: {reason}"))]
    Closed { code: CloseCode, reason: String },

    #[snafu(display("Driver closed the socket without reason"))]
    ClosedWithoutReason,

    #[snafu(display("Unexpected answer from the driver: {msg:#?}"))]
    UnexpectedAnswer { msg: Message },

    #[snafu(display("Unexpected end of websocket stream"))]
    End,

    #[snafu(display("Could not encode message: {source}"))]
    Encoding { source: serde_json::Error },

    #[snafu(display("Could not decode message: {source}"))]
    Decoding { source: serde_json::Error },

    #[snafu(display("{source}"))]
    Other { source: Box<dyn std::error::Error> },
}

impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(source: async_tungstenite::tungstenite::Error) -> Self {
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
    socket: async_tungstenite::WebSocketStream<ClientStream<async_net::TcpStream>>,
}

/// These are the core functionalities of `Session`.
impl Session {
    /// Create a new session by connecting to a running instance of the driver.
    #[tracing::instrument(skip_all)]
    pub async fn new(url: impl AsRef<str>) -> Result<Self, Error> {
        let request = url.as_ref().into_client_request()?;
        log::trace!("created request: {request:#?}");
        let host = request.uri().host().context(MissingHostSnafu)?;
        log::trace!("host: {host}");
        let port = request.uri().port().context(MissingPortSnafu)?;
        log::trace!("port: {port}");
        let path = request.uri().path();
        log::trace!("path: {path}");
        let mode = async_tungstenite::tungstenite::client::uri_mode(request.uri());
        log::trace!("mode: {mode:#?}");
        let tcp_stream = TcpStream::connect(format!("{host}:{port}"))
            .await
            .map_err(|e| Error::Other {
                source: Box::new(e),
            })?;
        log::trace!("created tcp_stream");
        let (socket, response) =
            async_tungstenite::async_tls::client_async_tls(request, tcp_stream).await?;
        log::trace!("handshake response: {response:#?}");
        Ok(Self { socket })
    }

    #[tracing::instrument(skip_all)]
    async fn recv(&mut self) -> Result<OutputMessage, Error> {
        match self
            .socket
            .next()
            .instrument(tracing::trace_span!("awaiting"))
            .await
            .context(EndSnafu)??
        {
            Message::Text(text) => {
                let msg: OutputMessage = tracing::info_span!("decoding-text")
                    .in_scope(|| serde_json::from_str(&text).context(DecodingSnafu))?;
                Ok(msg)
            }
            Message::Binary(vec) => {
                let msg: OutputMessage = tracing::info_span!("decoding-binary")
                    .in_scope(|| serde_json::from_slice(&vec).context(DecodingSnafu))?;
                Ok(msg)
            }
            Message::Close(maybe_frame) => match maybe_frame {
                Some(CloseFrame { code, reason }) => {
                    log::warn!("socket closed by the driver: {code} {reason}");
                    ClosedSnafu {
                        code,
                        reason: reason.to_string(),
                    }
                    .fail()
                }
                None => {
                    log::warn!("socket closed without reason");
                    ClosedWithoutReasonSnafu.fail()
                }
            },
            msg => UnexpectedAnswerSnafu { msg }.fail(),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn send(&mut self, msg: InputMessage) -> Result<OutputMessage, Error> {
        log::trace!("send: {msg:#?}");
        let text = tracing::trace_span!("encoding")
            .in_scope(|| serde_json::to_string(&msg).context(EncodingSnafu))?;
        let bytes = Utf8Bytes::from(text);
        self.socket
            .send(Message::Text(bytes))
            .instrument(tracing::trace_span!("sending"))
            .await?;
        let rmsg = self.recv().await?;
        log::trace!("recv: {rmsg:#?}");
        Ok(rmsg)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_screens(&mut self) -> Result<Vec<Screen>, Error> {
        txrx!(self, InputMessage::GetScreens, OutputMessage::GotScreens(screens) => screens)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_main_screen(&mut self) -> Result<Screen, Error> {
        txrx!(self, InputMessage::GetMainScreen, OutputMessage::GotMainScreen(screen) => screen)
    }

    #[tracing::instrument(skip_all)]
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

    #[tracing::instrument(skip(self))]
    pub async fn get_mouse_location(&mut self) -> Result<Vec2, Error> {
        txrx!(self, InputMessage::GetMouseLocation, OutputMessage::GotMouseLocation(loc) => loc)
    }

    #[tracing::instrument(skip(self))]
    pub async fn input(&mut self, token: Token) -> Result<(), Error> {
        txrx!(self, InputMessage::DoInput(token), OutputMessage::DidInput => ())
    }

    #[tracing::instrument(skip_all)]
    pub async fn text(&mut self, text: impl AsRef<str>) -> Result<(), Error> {
        txrx!(self, InputMessage::DoTypeText(text.as_ref().to_owned()), OutputMessage::DidTypeText => ())
    }

    #[tracing::instrument(skip(self, screen_name, text))]
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

    #[tracing::instrument(skip(self, screen_name))]
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
            OutputMessage::FoundImage { locations } => locations
        )
    }
}

/// These are higher-order functionalities of `Session`.
impl Session {
    /// Sleep for the duration.
    #[tracing::instrument(skip_all)]
    pub async fn sleep(&self, duration: impl Into<std::time::Duration>) {
        /// This is quite a quick and dirty implementation of sleep.
        ///
        /// This will be polled every frame until the timeout is reached.
        struct Sleep {
            instant: std::time::Instant,
            timeout: std::time::Duration,
        }

        impl std::future::Future for Sleep {
            type Output = ();

            fn poll(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                if self.instant.elapsed() >= self.timeout {
                    std::task::Poll::Ready(())
                } else {
                    // wake the waker to ensure this gets polled next frame
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            }
        }

        let timeout = duration.into();
        let instant = std::time::Instant::now();
        Sleep { instant, timeout }.await
    }

    /// Sleep for 10 milliseconds.
    #[tracing::instrument(skip_all)]
    pub async fn pause(&self) {
        self.sleep(std::time::Duration::from_millis(10)).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn key_down(&mut self, key: Key) -> Result<(), Error> {
        self.input(Token::Key(key, Direction::Press)).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn key_up(&mut self, key: Key) -> Result<(), Error> {
        self.input(Token::Key(key, Direction::Release)).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn key_click(&mut self, key: Key) -> Result<(), Error> {
        self.input(Token::Key(key, Direction::Click)).await
    }

    #[tracing::instrument(skip_all)]
    pub async fn type_text(&mut self, text: impl AsRef<str>) -> Result<(), Error> {
        self.input(Token::Text(text.as_ref().into())).await
    }

    /// Set the current location of the mouse in pixels.
    #[tracing::instrument(skip(self))]
    pub async fn set_mouse_location(
        &mut self,
        loc: Vec2,
        coordinate: Coordinate,
    ) -> Result<(), Error> {
        self.input(Token::MoveMouse(loc.x as i32, loc.y as i32, coordinate))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_left_down(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Left, Direction::Press))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_left_up(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Left, Direction::Release))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_left_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Left, Direction::Click))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_left_double_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Left, Direction::Click))
            .await?;
        self.pause().await;
        self.input(Token::Button(Button::Left, Direction::Click))
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_middle_down(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Middle, Direction::Press))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_middle_up(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Middle, Direction::Release))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_middle_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Middle, Direction::Click))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_middle_double_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Middle, Direction::Click))
            .await?;
        self.pause().await;
        self.input(Token::Button(Button::Middle, Direction::Click))
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_right_down(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Right, Direction::Press))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_right_up(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Right, Direction::Release))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_right_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Right, Direction::Click))
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn mouse_right_double_click(&mut self) -> Result<(), Error> {
        self.input(Token::Button(Button::Right, Direction::Click))
            .await?;
        self.pause().await;
        self.input(Token::Button(Button::Right, Direction::Click))
            .await?;
        Ok(())
    }

    /// Move the mouse along a path over some period of time.
    ///
    /// ## NOTE
    /// On MacOS, this requires the user to allowlist the application to control their computer
    /// through accessibility features.
    #[tracing::instrument(skip(self, path))]
    pub async fn mouse_path(
        &mut self,
        path: impl IntoIterator<Item = Vec2>,
        coordinate: Coordinate,
        time_in_seconds: f32,
    ) -> Result<(), Error> {
        let path = path.into_iter().collect::<Vec<_>>();
        let list = path.iter().zip(path.iter().skip(1)).collect::<Vec<_>>();
        let total_distance = list
            .iter()
            .map(|(from, to)| from.distance(**to))
            .sum::<f32>();
        for (from, to) in list.into_iter() {
            let distance = from.distance(*to);
            let percentage_of_path = distance / total_distance;
            let spline_time =
                std::time::Duration::from_secs_f32(time_in_seconds * percentage_of_path);
            let start = std::time::Instant::now();
            self.input(Token::MoveMouse(from.x as i32, from.y as i32, coordinate))
                .await?;
            let mut now = std::time::Instant::now();
            while now - start < spline_time {
                now = std::time::Instant::now();
                let percentage_done = (now - start).as_secs_f32() / spline_time.as_secs_f32();
                let position = from.lerp(*to, percentage_done);
                self.input(Token::MoveMouse(
                    position.x as i32,
                    position.y as i32,
                    coordinate,
                ))
                .await?;
            }
        }
        Ok(())
    }
}
