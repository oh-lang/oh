pub use crate::core::signed::Signed as Offset;

pub type Offset64 = Offset<i64>;
pub type Offset32 = Offset<i32>;
pub type Offset16 = Offset<i16>;
pub type Offset8 = Offset<i8>;

// TODO: Offset should be a wrapper like Count which becomes null if negative.

// TODO: switch to 32 on 32bit platforms
pub type OffsetMax = Offset64;
