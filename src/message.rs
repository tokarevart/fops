use std::ops::{Deref, DerefMut};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct Message(Vec<u8>);

impl<T: AsRef<[u8]>> From<T> for Message {
    fn from(value: T) -> Self {
        Message(value.as_ref().to_vec())
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        value.0
    }
}

impl Deref for Message {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
