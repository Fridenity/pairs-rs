use std::fmt;

#[derive(Debug)]
pub enum TitleButtons {
    Start,
    Options,
    Exit,
}

impl fmt::Display for TitleButtons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum InputMode {
    Normal,
    Action1,
    // Action2,
}

pub enum PopupSeverity {
    Info,
    Warn,
    Err,
}

pub struct PopupMsg {
    pub msg: String,
    pub severity: PopupSeverity,
}

impl PopupMsg {
    pub fn new(msg: String, severity: PopupSeverity) -> Self {
        Self { msg, severity }
    }

    pub fn info(msg: String) -> Self {
        Self::new(msg, PopupSeverity::Info)
    }

    pub fn warn(msg: String) -> Self {
        Self::new(msg, PopupSeverity::Warn)
    }

    pub fn err(msg: String) -> Self {
        Self::new(msg, PopupSeverity::Err)
    }
}

pub enum Screen {
    Title,
    PlayerCountInput,
    PlayerNameInput,
    Gameplay,
    Options,
}
