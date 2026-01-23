//! Shared enums for Systematics

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::Enum;

/// Language enum for vocabularies and representations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Enum))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    /// Standard Elementary Systematics vocabulary
    Canonical,
    /// Energy-based vocabulary (affirming, denying, reconciling)
    Energy,
    /// Values-based vocabulary
    Values,
    /// Social vocabulary framework
    Society,
    /// Hexadecimal color representation
    Hex,
    /// Named color representation
    Name,
}

impl Language {
    /// Check if this is a vocabulary language (for Character entries)
    pub fn is_vocabulary(&self) -> bool {
        matches!(
            self,
            Language::Canonical | Language::Energy | Language::Values | Language::Society
        )
    }

    /// Check if this is a representation language (for Colour entries)
    pub fn is_representation(&self) -> bool {
        matches!(self, Language::Hex | Language::Name)
    }

    /// Get all vocabulary languages
    pub fn all_vocabularies() -> [Language; 4] {
        [
            Language::Canonical,
            Language::Energy,
            Language::Values,
            Language::Society,
        ]
    }

    /// Get all representation languages
    pub fn all_representations() -> [Language; 2] {
        [Language::Hex, Language::Name]
    }
}

/// Link type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Enum))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkType {
    /// Geometric edge between coordinates
    Line,
    /// Semantic connection between terms
    Connective,
}
