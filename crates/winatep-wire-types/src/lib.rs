//! Wire types.

pub mod key;

/// Information about an operating system screen or monitor.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Screen {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub scale_factor: f32,
}

impl Screen {
    pub fn bounds(&self) -> (glam::Vec2, glam::Vec2) {
        let x = self.x as f32;
        let y = self.y as f32;
        let width = self.width as f32;
        let height = self.height as f32;
        (
            glam::Vec2::new(x, y),
            glam::Vec2::new(x + width, y + height),
        )
    }
}

/// A server status message.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum ServerStatus<T> {
    #[default]
    Off,
    On {
        #[serde(skip)]
        handle: T,
        port: u32,
    },
}

impl<T> ServerStatus<T> {
    pub fn port(&self) -> Option<u32> {
        match self {
            ServerStatus::On { port, .. } => Some(*port),
            _ => None,
        }
    }

    pub fn is_on(&self) -> bool {
        self.port().is_some()
    }
}

/// An image buffer.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl core::fmt::Debug for ImageBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageBuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("buffer", &format!("{}bytes", self.buffer.len()))
            .finish()
    }
}

/// The direction of a key or button.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    #[serde(alias = "P")]
    #[serde(alias = "p")]
    #[serde(alias = "Pressed")]
    #[serde(alias = "pressed")]
    Press,
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    #[serde(alias = "Released")]
    #[serde(alias = "released")]
    Release,
    /// Equivalent to a press followed by a release
    #[serde(alias = "C")]
    #[serde(alias = "c")]
    #[serde(alias = "Clicked")]
    #[serde(alias = "clicked")]
    #[default]
    Click,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    /// Left mouse button
    #[serde(alias = "L")]
    #[serde(alias = "l")]
    #[default]
    Left,
    /// Middle mouse button
    #[serde(alias = "M")]
    #[serde(alias = "m")]
    Middle,
    /// Right mouse button
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Right,
    /// 4th mouse button. Typically performs the same function as `Browser_Back`
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    #[serde(alias = "B")]
    #[serde(alias = "b")]
    Back,
    /// 5th mouse button. Typically performs the same function as
    /// `Browser_Forward`
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    #[serde(alias = "F")]
    #[serde(alias = "f")]
    Forward,

    /// Scroll up button. It is better to use the
    /// [`Mouse::scroll`] method to scroll.
    #[serde(alias = "SU")]
    #[serde(alias = "su")]
    ScrollUp,
    /// Scroll down button. It is better to use the
    /// [`Mouse::scroll`] method to scroll.
    #[serde(alias = "SD")]
    #[serde(alias = "sd")]
    ScrollDown,
    /// Scroll left button. It is better to use the
    /// [`Mouse::scroll`] method to scroll.
    #[serde(alias = "SL")]
    #[serde(alias = "sl")]
    ScrollLeft,
    /// Scroll right button. It is better to use the
    /// [`Mouse::scroll`] method to scroll.
    #[serde(alias = "SR")]
    #[serde(alias = "sr")]
    ScrollRight,
}

/// Specifies if a coordinate is relative or absolute
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum Coordinate {
    #[doc(alias = "Absolute")]
    #[serde(alias = "A")]
    #[serde(alias = "a")]
    #[default]
    Abs,
    #[doc(alias = "Relative")]
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Rel,
}

/// Specifies the axis for scrolling
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum Axis {
    #[serde(alias = "H")]
    #[serde(alias = "h")]
    Horizontal,
    #[serde(alias = "V")]
    #[serde(alias = "v")]
    #[default]
    Vertical,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    /// Call the [`Keyboard::text`] fn with the string as text
    #[serde(alias = "T")]
    #[serde(alias = "t")]
    Text(String),
    /// Call the [`Keyboard::key`] fn with the given key and direction
    #[serde(alias = "K")]
    #[serde(alias = "k")]
    Key(key::Key, #[serde(default)] Direction),
    /// Call the [`Keyboard::raw`] fn with the given keycode and direction
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Raw(u16, #[serde(default)] Direction),
    /// Call the [`Mouse::button`] fn with the given mouse button and direction
    #[serde(alias = "B")]
    #[serde(alias = "b")]
    Button(Button, #[serde(default)] Direction),
    /// Call the [`Mouse::move_mouse`] fn. The first i32 is the value to move on
    /// the x-axis and the second i32 is the value to move on the y-axis. The
    /// coordinate defines if the given coordinates are absolute of relative to
    /// the current position of the mouse.
    #[serde(alias = "M")]
    #[serde(alias = "m")]
    MoveMouse(i32, i32, #[serde(default)] Coordinate),
    /// Call the [`Mouse::scroll`] fn.
    #[serde(alias = "S")]
    #[serde(alias = "s")]
    Scroll(i32, #[serde(default)] Axis),
    /// Call the [`Mouse::location`] fn and compare the return values with
    /// the values of this enum. Log an error if they are not equal.
    /// This variant contains the EXPECTED location of the mouse
    #[serde(alias = "L")]
    #[serde(alias = "l")]
    Location(i32, i32),
    /// Call the [`Mouse::main_display`] fn and compare the return values with
    /// the values of this enum. Log an error if they are not equal.
    /// This variant contains the EXPECTED size of the main display
    #[serde(alias = "D")]
    #[serde(alias = "d")]
    MainDisplay(i32, i32),
}

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BoundingRectangle {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

/// Websocket driver input messages.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InputMessage {
    GetScreens,
    GetMainScreen,
    CaptureScreen {
        name: String,
    },
    GetMouseLocation,
    DoInput(Token),
    FindText {
        text: String,
        screen_name: String,
        timeout_in_seconds: f32,
    },
}

/// Websocket driver output messages.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OutputMessage {
    GotScreens(Vec<Screen>),
    GotMainScreen(Screen),
    CapturedScreen { image_buffer: ImageBuffer },
    GotMouseLocation(f32, f32),
    DidInput,
    FoundText { locations: Vec<BoundingRectangle> },
    Error(String),
}
