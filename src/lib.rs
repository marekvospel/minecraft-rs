pub mod error;
pub mod game_state;
pub mod legacy;
pub mod packets;
pub mod var_int;

pub(crate) type Result<T> = crate::error::Result<T>;
