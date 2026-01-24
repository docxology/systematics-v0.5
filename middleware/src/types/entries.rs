//! Entry types for Systematics wire format

use super::Language;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// Character - a reusable vocabulary element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Character {
    pub id: String,
    pub language: Language,
    pub value: String,
}

/// Term - a positional entry with character reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Term {
    pub id: String,
    pub order: i32,
    pub position: i32,
    #[serde(rename = "characterId")]
    pub character_id: String,
    pub character: Option<Character>,
}

/// Coordinate - a 3D point at a specific location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Coordinate {
    pub id: String,
    pub order: i32,
    pub position: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Colour - a color value at a specific location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Colour {
    pub id: String,
    pub order: i32,
    pub position: i32,
    pub language: Language,
    pub value: String,
}

/// Slice - all entries at a specific order+position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Slice {
    pub order: i32,
    pub position: i32,
    pub term: Option<Term>,
    pub coordinate: Option<Coordinate>,
    pub colour: Option<Colour>,
}
