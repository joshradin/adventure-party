use naia_shared::BigMapKey;

/// An entity
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Entity(u64);

impl BigMapKey for Entity {
    fn to_u64(&self) -> u64 {
        self.0
    }

    fn from_u64(value: u64) -> Self {
        Self(value)
    }
}
