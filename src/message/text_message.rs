use std::ops::{Deref, DerefMut};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct TextMessage(String);

impl<T: AsRef<str>> From<T> for TextMessage {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl From<TextMessage> for String {
    fn from(value: TextMessage) -> Self {
        value.0
    }
}

impl Deref for TextMessage {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
