//! System view types for Systematics wire format

use serde::{Deserialize, Serialize};
use super::{Term, Coordinate, Colour, Link};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// SystemView - a complete view of a system at a given order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct SystemView {
    pub order: i32,
    pub name: Option<String>,
    pub coherence: Option<String>,
    #[serde(rename = "termDesignation")]
    pub term_designation: Option<String>,
    #[serde(rename = "connectiveDesignation")]
    pub connective_designation: Option<String>,
    pub terms: Vec<Term>,
    pub coordinates: Vec<Coordinate>,
    pub colours: Vec<Colour>,
    pub connectives: Vec<Link>,
    pub lines: Vec<Link>,
    /// All links (both lines and connectives)
    #[serde(default)]
    pub links: Vec<Link>,
}

impl SystemView {
    /// Get the system name, falling back to order-based name
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            match self.order {
                1 => "Monad",
                2 => "Dyad",
                3 => "Triad",
                4 => "Tetrad",
                5 => "Pentad",
                6 => "Hexad",
                7 => "Heptad",
                8 => "Octad",
                9 => "Ennead",
                10 => "Decad",
                11 => "Undecad",
                12 => "Dodecad",
                _ => "Unknown",
            }.to_string()
        })
    }

    /// Get the K-notation for this system (e.g., "K3" for Triad)
    pub fn k_notation(&self) -> String {
        format!("K{}", self.order)
    }

    /// Get the description/coherence for this system
    pub fn description(&self) -> String {
        self.coherence.clone().unwrap_or_else(|| self.display_name())
    }

    /// Get the number of nodes in this system
    pub fn node_count(&self) -> usize {
        self.order as usize
    }

    /// Get the term value at a position (1-based)
    pub fn term_at(&self, position: i32) -> Option<&str> {
        self.terms.iter()
            .find(|t| t.position == position)
            .and_then(|t| t.character.as_ref())
            .map(|c| c.value.as_str())
    }

    /// Get the colour value at a position (1-based)
    pub fn colour_at(&self, position: i32) -> Option<&str> {
        self.colours.iter()
            .find(|c| c.position == position)
            .map(|c| c.value.as_str())
    }

    /// Get the coordinate at a position (1-based)
    pub fn coordinate_at(&self, position: i32) -> Option<&Coordinate> {
        self.coordinates.iter()
            .find(|c| c.position == position)
    }
}
