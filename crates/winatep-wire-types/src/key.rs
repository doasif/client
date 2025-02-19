//! Keys on the keyboard.

/// Use [`Key::Unicode`] to enter arbitrary Unicode chars.
/// If a key is missing, you can simulate that key by using [`Key::Other`]
/// or the [`crate::Keyboard::raw`] function. Some of the keys are only
/// available on a specific platform. Use conditional compilation to use them.
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    #[cfg(target_os = "windows")]
    Num0,
    #[cfg(target_os = "windows")]
    Num1,
    #[cfg(target_os = "windows")]
    Num2,
    #[cfg(target_os = "windows")]
    Num3,
    #[cfg(target_os = "windows")]
    Num4,
    #[cfg(target_os = "windows")]
    Num5,
    #[cfg(target_os = "windows")]
    Num6,
    #[cfg(target_os = "windows")]
    Num7,
    #[cfg(target_os = "windows")]
    Num8,
    #[cfg(target_os = "windows")]
    Num9,
    #[cfg(target_os = "windows")]
    A,
    #[cfg(target_os = "windows")]
    B,
    #[cfg(target_os = "windows")]
    C,
    #[cfg(target_os = "windows")]
    D,
    #[cfg(target_os = "windows")]
    E,
    #[cfg(target_os = "windows")]
    F,
    #[cfg(target_os = "windows")]
    G,
    #[cfg(target_os = "windows")]
    H,
    #[cfg(target_os = "windows")]
    I,
    #[cfg(target_os = "windows")]
    J,
    #[cfg(target_os = "windows")]
    K,
    #[cfg(target_os = "windows")]
    L,
    #[cfg(target_os = "windows")]
    M,
    #[cfg(target_os = "windows")]
    N,
    #[cfg(target_os = "windows")]
    O,
    #[cfg(target_os = "windows")]
    P,
    #[cfg(target_os = "windows")]
    Q,
    #[cfg(target_os = "windows")]
    R,
    #[cfg(target_os = "windows")]
    S,
    #[cfg(target_os = "windows")]
    T,
    #[cfg(target_os = "windows")]
    U,
    #[cfg(target_os = "windows")]
    V,
    #[cfg(target_os = "windows")]
    W,
    #[cfg(target_os = "windows")]
    X,
    #[cfg(target_os = "windows")]
    Y,
    #[cfg(target_os = "windows")]
    Z,
    #[cfg(target_os = "windows")]
    AbntC1,
    #[cfg(target_os = "windows")]
    AbntC2,
    #[cfg(target_os = "windows")]
    Accept,
    #[cfg(target_os = "windows")]
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(target_os = "windows")]
    Apps,
    #[cfg(target_os = "windows")]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(all(unix, not(target_os = "macos")))]
    Break,
    #[cfg(all(unix, not(target_os = "macos")))]
    Begin,
    #[cfg(target_os = "macos")]
    BrightnessDown,
    #[cfg(target_os = "macos")]
    BrightnessUp,
    #[cfg(target_os = "windows")]
    BrowserBack,
    #[cfg(target_os = "windows")]
    BrowserFavorites,
    #[cfg(target_os = "windows")]
    BrowserForward,
    #[cfg(target_os = "windows")]
    BrowserHome,
    #[cfg(target_os = "windows")]
    BrowserRefresh,
    #[cfg(target_os = "windows")]
    BrowserSearch,
    #[cfg(target_os = "windows")]
    BrowserStop,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Clear,
    #[cfg(target_os = "macos")]
    ContrastUp,
    #[cfg(target_os = "macos")]
    ContrastDown,
    /// control key
    #[serde(alias = "ctrl")]
    Control,
    #[cfg(target_os = "windows")]
    Convert,
    #[cfg(target_os = "windows")]
    Crsel,
    #[cfg(target_os = "windows")]
    DBEAlphanumeric,
    #[cfg(target_os = "windows")]
    DBECodeinput,
    #[cfg(target_os = "windows")]
    DBEDetermineString,
    #[cfg(target_os = "windows")]
    DBEEnterDLGConversionMode,
    #[cfg(target_os = "windows")]
    DBEEnterIMEConfigMode,
    #[cfg(target_os = "windows")]
    DBEEnterWordRegisterMode,
    #[cfg(target_os = "windows")]
    DBEFlushString,
    #[cfg(target_os = "windows")]
    DBEHiragana,
    #[cfg(target_os = "windows")]
    DBEKatakana,
    #[cfg(target_os = "windows")]
    DBENoCodepoint,
    #[cfg(target_os = "windows")]
    DBENoRoman,
    #[cfg(target_os = "windows")]
    DBERoman,
    #[cfg(target_os = "windows")]
    DBESBCSChar,
    #[cfg(target_os = "windows")]
    DBESChar,
    #[cfg(target_os = "windows")]
    Decimal,
    /// delete key
    Delete,
    #[cfg(target_os = "windows")]
    Divide,
    /// down arrow key
    DownArrow,
    #[cfg(target_os = "macos")]
    Eject,
    /// end key
    End,
    #[cfg(target_os = "windows")]
    Ereof,
    /// escape key (esc)
    Escape,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Execute,
    #[cfg(target_os = "windows")]
    Exsel,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    /// F21 key
    F21,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    /// F22 key
    F22,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    /// F23 key
    F23,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    /// F24 key
    F24,
    #[cfg(all(unix, not(target_os = "macos")))]
    F25,
    #[cfg(all(unix, not(target_os = "macos")))]
    F26,
    #[cfg(all(unix, not(target_os = "macos")))]
    F27,
    #[cfg(all(unix, not(target_os = "macos")))]
    F28,
    #[cfg(all(unix, not(target_os = "macos")))]
    F29,
    #[cfg(all(unix, not(target_os = "macos")))]
    F30,
    #[cfg(all(unix, not(target_os = "macos")))]
    F31,
    #[cfg(all(unix, not(target_os = "macos")))]
    F32,
    #[cfg(all(unix, not(target_os = "macos")))]
    F33,
    #[cfg(all(unix, not(target_os = "macos")))]
    F34,
    #[cfg(all(unix, not(target_os = "macos")))]
    F35,
    #[cfg(target_os = "macos")]
    Function,
    #[cfg(target_os = "windows")]
    Final,
    #[cfg(all(unix, not(target_os = "macos")))]
    Find,
    #[cfg(target_os = "windows")]
    GamepadA,
    #[cfg(target_os = "windows")]
    GamepadB,
    #[cfg(target_os = "windows")]
    GamepadDPadDown,
    #[cfg(target_os = "windows")]
    GamepadDPadLeft,
    #[cfg(target_os = "windows")]
    GamepadDPadRight,
    #[cfg(target_os = "windows")]
    GamepadDPadUp,
    #[cfg(target_os = "windows")]
    GamepadLeftShoulder,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickButton,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickDown,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickLeft,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickRight,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickUp,
    #[cfg(target_os = "windows")]
    GamepadLeftTrigger,
    #[cfg(target_os = "windows")]
    GamepadMenu,
    #[cfg(target_os = "windows")]
    GamepadRightShoulder,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickButton,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickDown,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickLeft,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickRight,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickUp,
    #[cfg(target_os = "windows")]
    GamepadRightTrigger,
    #[cfg(target_os = "windows")]
    GamepadView,
    #[cfg(target_os = "windows")]
    GamepadX,
    #[cfg(target_os = "windows")]
    GamepadY,
    #[cfg(target_os = "windows")]
    Hangeul,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Hangul,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(target_os = "windows")]
    Ico00,
    #[cfg(target_os = "windows")]
    IcoClear,
    #[cfg(target_os = "windows")]
    IcoHelp,
    #[cfg(target_os = "macos")]
    IlluminationDown,
    #[cfg(target_os = "macos")]
    IlluminationUp,
    #[cfg(target_os = "macos")]
    IlluminationToggle,
    #[cfg(target_os = "windows")]
    IMEOff,
    #[cfg(target_os = "windows")]
    IMEOn,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Insert,
    #[cfg(target_os = "windows")]
    Junja,
    #[cfg(target_os = "windows")]
    Kana,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Kanji,
    #[cfg(target_os = "windows")]
    LaunchApp1,
    #[cfg(target_os = "windows")]
    LaunchApp2,
    #[cfg(target_os = "windows")]
    LaunchMail,
    #[cfg(target_os = "windows")]
    LaunchMediaSelect,
    #[cfg(target_os = "macos")]
    /// Opens launchpad
    Launchpad,
    #[cfg(target_os = "macos")]
    LaunchPanel,
    #[cfg(target_os = "windows")]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(all(unix, not(target_os = "macos")))]
    Linefeed,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    LMenu,
    LShift,
    #[cfg(target_os = "windows")]
    LWin,
    #[cfg(target_os = "windows")]
    MButton,
    #[cfg(target_os = "macos")]
    MediaFast,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    #[cfg(target_os = "macos")]
    MediaRewind,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(target_os = "macos")]
    /// Opens mission control
    MissionControl,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    ModeChange,
    #[cfg(target_os = "windows")]
    Multiply,
    #[cfg(target_os = "windows")]
    NavigationAccept,
    #[cfg(target_os = "windows")]
    NavigationCancel,
    #[cfg(target_os = "windows")]
    NavigationDown,
    #[cfg(target_os = "windows")]
    NavigationLeft,
    #[cfg(target_os = "windows")]
    NavigationMenu,
    #[cfg(target_os = "windows")]
    NavigationRight,
    #[cfg(target_os = "windows")]
    NavigationUp,
    #[cfg(target_os = "windows")]
    NavigationView,
    #[cfg(target_os = "windows")]
    NoName,
    #[cfg(target_os = "windows")]
    NonConvert,
    #[cfg(target_os = "windows")]
    None,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Numlock,
    #[cfg(target_os = "windows")]
    Numpad0,
    #[cfg(target_os = "windows")]
    Numpad1,
    #[cfg(target_os = "windows")]
    Numpad2,
    #[cfg(target_os = "windows")]
    Numpad3,
    #[cfg(target_os = "windows")]
    Numpad4,
    #[cfg(target_os = "windows")]
    Numpad5,
    #[cfg(target_os = "windows")]
    Numpad6,
    #[cfg(target_os = "windows")]
    Numpad7,
    #[cfg(target_os = "windows")]
    Numpad8,
    #[cfg(target_os = "windows")]
    Numpad9,
    #[cfg(target_os = "windows")]
    OEM1,
    #[cfg(target_os = "windows")]
    OEM102,
    #[cfg(target_os = "windows")]
    OEM2,
    #[cfg(target_os = "windows")]
    OEM3,
    #[cfg(target_os = "windows")]
    OEM4,
    #[cfg(target_os = "windows")]
    OEM5,
    #[cfg(target_os = "windows")]
    OEM6,
    #[cfg(target_os = "windows")]
    OEM7,
    #[cfg(target_os = "windows")]
    OEM8,
    #[cfg(target_os = "windows")]
    OEMAttn,
    #[cfg(target_os = "windows")]
    OEMAuto,
    #[cfg(target_os = "windows")]
    OEMAx,
    #[cfg(target_os = "windows")]
    OEMBacktab,
    #[cfg(target_os = "windows")]
    OEMClear,
    #[cfg(target_os = "windows")]
    OEMComma,
    #[cfg(target_os = "windows")]
    OEMCopy,
    #[cfg(target_os = "windows")]
    OEMCusel,
    #[cfg(target_os = "windows")]
    OEMEnlw,
    #[cfg(target_os = "windows")]
    OEMFinish,
    #[cfg(target_os = "windows")]
    OEMFJJisho,
    #[cfg(target_os = "windows")]
    OEMFJLoya,
    #[cfg(target_os = "windows")]
    OEMFJMasshou,
    #[cfg(target_os = "windows")]
    OEMFJRoya,
    #[cfg(target_os = "windows")]
    OEMFJTouroku,
    #[cfg(target_os = "windows")]
    OEMJump,
    #[cfg(target_os = "windows")]
    OEMMinus,
    #[cfg(target_os = "windows")]
    OEMNECEqual,
    #[cfg(target_os = "windows")]
    OEMPA1,
    #[cfg(target_os = "windows")]
    OEMPA2,
    #[cfg(target_os = "windows")]
    OEMPA3,
    #[cfg(target_os = "windows")]
    OEMPeriod,
    #[cfg(target_os = "windows")]
    OEMPlus,
    #[cfg(target_os = "windows")]
    OEMReset,
    #[cfg(target_os = "windows")]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(target_os = "windows")]
    PA1,
    #[cfg(target_os = "windows")]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Pause,
    #[cfg(target_os = "windows")]
    Play,
    #[cfg(target_os = "macos")]
    Power,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Print,
    /// Take a screenshot
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    #[doc(alias = "Print")]
    #[doc(alias = "Snapshot")]
    PrintScr,
    #[cfg(target_os = "windows")]
    Processkey,
    #[cfg(target_os = "windows")]
    RButton,
    #[cfg(target_os = "macos")]
    RCommand,
    RControl,
    #[cfg(all(unix, not(target_os = "macos")))]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(target_os = "windows")]
    RMenu,
    #[cfg(target_os = "macos")]
    ROption,
    RShift,
    #[cfg(target_os = "windows")]
    RWin,
    #[cfg(target_os = "windows")]
    Scroll,
    #[cfg(all(unix, not(target_os = "macos")))]
    ScrollLock,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    Select,
    #[cfg(all(unix, not(target_os = "macos")))]
    ScriptSwitch,
    #[cfg(target_os = "windows")]
    Separator,
    /// shift key
    Shift,
    #[cfg(all(unix, not(target_os = "macos")))]
    /// Lock shift key
    ShiftLock,
    #[cfg(target_os = "windows")]
    Sleep,
    #[cfg(target_os = "windows")]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Snapshot,
    /// space key
    Space,
    #[cfg(target_os = "windows")]
    Subtract,
    #[cfg(all(unix, not(target_os = "macos")))]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(all(unix, not(target_os = "macos")))]
    Undo,
    /// up arrow key
    UpArrow,
    #[cfg(target_os = "macos")]
    VidMirror,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    #[cfg(all(unix, not(target_os = "macos")))]
    /// microphone mute toggle on linux
    MicMute,
    #[cfg(target_os = "windows")]
    XButton1,
    #[cfg(target_os = "windows")]
    XButton2,
    #[cfg(target_os = "windows")]
    Zoom,
    /// Unicode character
    #[doc(alias = "Layout")]
    #[serde(alias = "uni")]
    #[serde(alias = "Uni")]
    #[serde(alias = "Char")]
    #[serde(alias = "char")]
    Unicode(char),
    /// Use this for keys that are not listed here that you know the
    /// value of. Let us know if you think the key should be listed so
    /// we can add it
    /// On Linux, this will result in a keysym,
    /// On Windows, this will result in a `Virtual_Key` and
    /// On macOS, this will yield a `KeyCode`
    Other(u32),
}
