//! Shared wire format types for Systematics API
//!
//! These types define the JSON structure exchanged between backend and frontend.
//! They support both serialization (backend) and deserialization (frontend).

mod enums;
mod entries;
mod links;
mod system;
mod error;

pub use enums::*;
pub use entries::*;
pub use links::*;
pub use system::*;
pub use error::*;
