//! Link types for Systematics wire format

use super::{Character, Coordinate, LinkType};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// Link - a connection between entries (either Line or Connective)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Link {
    pub id: String,
    #[serde(rename = "baseId")]
    pub base_id: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "linkType")]
    pub link_type: LinkType,
    #[serde(rename = "characterId")]
    pub character_id: Option<String>,
    pub tag: Option<String>,
    pub order: Option<i32>,
    #[serde(rename = "basePosition")]
    pub base_position: Option<i32>,
    #[serde(rename = "targetPosition")]
    pub target_position: Option<i32>,
    /// Resolved character for this link (for connectives)
    pub character: Option<Character>,
    /// Resolved base coordinate
    #[serde(rename = "baseCoordinate")]
    pub base_coordinate: Option<Coordinate>,
    /// Resolved target coordinate
    #[serde(rename = "targetCoordinate")]
    pub target_coordinate: Option<Coordinate>,
}
