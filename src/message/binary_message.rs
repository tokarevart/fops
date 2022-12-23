use std::ops::{Deref, DerefMut};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct BinaryMessage(Vec<u8>);

impl<T: AsRef<[u8]>> From<T> for BinaryMessage {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_vec())
    }
}

impl From<BinaryMessage> for Vec<u8> {
    fn from(value: BinaryMessage) -> Self {
        value.0
    }
}

impl Deref for BinaryMessage {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BinaryMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
