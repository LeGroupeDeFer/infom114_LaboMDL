use crate::lib::{consequence as conseq, Consequence, PostError};
use std::convert::TryFrom;

#[derive(PartialEq)]
pub enum PostKind {
    Info,
    Poll,
    Idea,
    Decision,
    Discussion,
}

impl TryFrom<u8> for PostKind {
    type Error = conseq::Error;

    fn try_from(n: u8) -> Consequence<PostKind> {
        Ok(match n {
            0 => PostKind::Info,
            1 => PostKind::Idea,
            2 => PostKind::Poll,
            3 => PostKind::Decision,
            4 => PostKind::Discussion,
            _ => Err(PostError::UnknownKind)?,
        })
    }
}

impl TryFrom<String> for PostKind {
    type Error = conseq::Error;

    fn try_from(s: String) -> Consequence<PostKind> {
        Ok(match &*s.to_lowercase() {
            "poll" => PostKind::Poll,
            "idea" => PostKind::Idea,
            "info" => PostKind::Info,
            "decision" => PostKind::Decision,
            "discussion" => PostKind::Discussion,
            _ => Err(PostError::UnknownKind)?,
        })
    }
}

impl From<PostKind> for u8 {
    fn from(kind: PostKind) -> u8 {
        u8::from(&kind)
    }
}

impl From<&PostKind> for u8 {
    fn from(kind: &PostKind) -> u8 {
        match kind {
            PostKind::Info => 0,
            PostKind::Idea => 1,
            PostKind::Poll => 2,
            PostKind::Decision => 3,
            PostKind::Discussion => 4,
        }
    }
}

impl From<PostKind> for String {
    fn from(kind: PostKind) -> String {
        String::from(&kind)
    }
}
impl From<&PostKind> for String {
    fn from(kind: &PostKind) -> String {
        match kind {
            PostKind::Info => "info".into(),
            PostKind::Idea => "idea".into(),
            PostKind::Poll => "poll".into(),
            PostKind::Decision => "decision".into(),
            PostKind::Discussion => "discussion".into(),
        }
    }
}