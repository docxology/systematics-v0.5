//! Language enum for semantic vocabularies and representation types.
//!
//! This unified enum covers both:
//! - Semantic vocabularies for Character entries (Canonical, Energy, Values, Society)
//! - Representation types for Colour entries (Hex, Name)

use serde::{Deserialize, Serialize};

/// Language represents either a semantic vocabulary or a representation type.
///
/// For Character entries: Canonical, Energy, Values, Society
/// For Colour entries: Hex, Name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    // Semantic vocabularies (for Character entries)
    /// The canonical/standard vocabulary from Elementary Systematics
    Canonical,
    /// Energy-based vocabulary (affirming, denying, reconciling)
    Energy,
    /// Values-based vocabulary
    Values,
    /// Society/social vocabulary
    Society,

    // Representation types (for Colour entries)
    /// Hexadecimal color representation (e.g., "#FF0000")
    Hex,
    /// Named color representation (e.g., "Red")
    Name,
}

impl Language {
    /// Returns true if this language is a semantic vocabulary (for Characters)
    pub fn is_vocabulary(&self) -> bool {
        matches!(
            self,
            Language::Canonical | Language::Energy | Language::Values | Language::Society
        )
    }

    /// Returns true if this language is a representation type (for Colours)
    pub fn is_representation(&self) -> bool {
        matches!(self, Language::Hex | Language::Name)
    }

    /// Get all vocabulary languages
    pub fn vocabularies() -> &'static [Language] {
        &[
            Language::Canonical,
            Language::Energy,
            Language::Values,
            Language::Society,
        ]
    }

    /// Get all representation languages
    pub fn representations() -> &'static [Language] {
        &[Language::Hex, Language::Name]
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Canonical => write!(f, "Canonical"),
            Language::Energy => write!(f, "Energy"),
            Language::Values => write!(f, "Values"),
            Language::Society => write!(f, "Society"),
            Language::Hex => write!(f, "Hex"),
            Language::Name => write!(f, "Name"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_classification() {
        assert!(Language::Canonical.is_vocabulary());
        assert!(Language::Energy.is_vocabulary());
        assert!(!Language::Hex.is_vocabulary());
    }

    #[test]
    fn test_representation_classification() {
        assert!(Language::Hex.is_representation());
        assert!(Language::Name.is_representation());
        assert!(!Language::Canonical.is_representation());
    }
}
