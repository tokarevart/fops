use std::ops::{Deref, DerefMut};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct BinaryMsg(Vec<u8>);

impl<T: AsRef<[u8]>> From<T> for BinaryMsg {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_vec())
    }
}

impl From<BinaryMsg> for Vec<u8> {
    fn from(value: BinaryMsg) -> Self {
        value.0
    }
}

impl Deref for BinaryMsg {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BinaryMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
