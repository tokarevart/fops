use std::ops::{Deref, DerefMut};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct TextMsg(String);

impl<T: AsRef<str>> From<T> for TextMsg {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl From<TextMsg> for String {
    fn from(value: TextMsg) -> Self {
        value.0
    }
}

impl Deref for TextMsg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
