use std::convert::TryFrom;
use crate::lib::{self as conseq, Consequence, WatchEventError};

pub enum WatchEventKind {
    Submit,
    Accept,
    Refuse,
    Progress,
    Finish
}

impl TryFrom<u8> for WatchEventKind {
    type Error = conseq::Error;

    fn try_from(n: u8) -> Consequence<WatchEventKind> {
        Ok(match n {
            1 => WatchEventKind::Submit,
            2 => WatchEventKind::Accept,
            3 => WatchEventKind::Refuse,
            4 => WatchEventKind::Progress,
            5 => WatchEventKind::Finish,
            _ => Err(WatchEventError::UnknownKind)?,
        })
    }
}

impl TryFrom<String> for WatchEventKind {
    type Error = conseq::Error;

    fn try_from(s: String) -> Consequence<WatchEventKind> {
        Ok(match &*s.to_lowercase() {
            "submit" => WatchEventKind::Submit,
            "accept" => WatchEventKind::Accept,
            "refuse" => WatchEventKind::Refuse,
            "progress" => WatchEventKind::Progress,
            "finish" => WatchEventKind::Finish,
            _ => Err(WatchEventError::UnknownKind)?,
        })
    }
}

impl From<WatchEventKind> for u8 {
    fn from(kind: WatchEventKind) -> u8 {
        u8::from(&kind)
    }
}

impl From<&WatchEventKind> for u8 {
    fn from(kind: &WatchEventKind) -> u8 {
        match kind {
            WatchEventKind::Submit => 1,
            WatchEventKind::Accept => 2,
            WatchEventKind::Refuse => 3,
            WatchEventKind::Progress => 4,
            WatchEventKind::Finish => 5,
        }
    }
}

impl From<WatchEventKind> for String {
    fn from(kind: WatchEventKind) -> String {
        String::from(&kind)
    }
}

impl From<&WatchEventKind> for String {
    fn from(kind: &WatchEventKind) -> String {
        match kind {
            WatchEventKind::Submit => "submit".into(),
            WatchEventKind::Accept => "accept".into(),
            WatchEventKind::Refuse => "refuse".into(),
            WatchEventKind::Progress => "progress".into(),
            WatchEventKind::Finish => "finish".into(),
        }
    }
}
