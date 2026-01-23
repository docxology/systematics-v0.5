//! Shared wire format types for Systematics API
//!
//! These types define the JSON structure exchanged between backend and frontend.
//! They support both serialization (backend) and deserialization (frontend).

mod entries;
mod enums;
mod error;
mod links;
mod system;

pub use entries::*;
pub use enums::*;
pub use error::*;
pub use links::*;
pub use system::*;
