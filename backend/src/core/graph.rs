//! Graph structure and query methods.
//!
//! The Graph is the primary container for the property graph,
//! holding all entries and links with methods for querying.
//!
//! Queries are organized into two categories:
//! - **Anchor Queries**: Query the fundamental graph structure (Order, Position, Location)
//! - **Systematic Queries**: Query semantic/categorical content mapped to anchors

use serde::{Deserialize, Serialize};

use super::entries::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, Entry, Location,
    Order, Position, SystemName, Term, TermDesignation,
};
use super::language::Language;
use super::links::{Link, LinkType};

/// Graph is the primary container for the property graph (AD4M: Perspective).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub entries: Vec<Entry>,
    pub links: Vec<Link>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry to the graph
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Add a link to the graph
    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Find an entry by ID
    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id() == id)
    }

    /// Find a link by ID
    pub fn get_link(&self, id: &str) -> Option<&Link> {
        self.links.iter().find(|l| l.id == id)
    }

    // ==========================================================================
    // Anchor Queries - Query the fundamental graph structure
    // ==========================================================================

    /// Get an Order entry by value
    pub fn order(&self, value: u8) -> Option<&Order> {
        self.entries.iter().find_map(|e| match e {
            Entry::Order(o) if o.value == value => Some(o),
            _ => None,
        })
    }

    /// Get all Order entries
    pub fn orders(&self) -> Vec<&Order> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Order(o) => Some(o),
                _ => None,
            })
            .collect()
    }

    /// Get a Position entry by value
    pub fn position(&self, value: u8) -> Option<&Position> {
        self.entries.iter().find_map(|e| match e {
            Entry::Position(p) if p.value == value => Some(p),
            _ => None,
        })
    }

    /// Get all Position entries
    pub fn positions(&self) -> Vec<&Position> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Position(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    /// Get a Location entry by order and position values
    pub fn location(&self, order: u8, position: u8) -> Option<&Location> {
        let order_id = format!("order_{}", order);
        let position_id = format!("position_{}", position);
        self.entries.iter().find_map(|e| match e {
            Entry::Location(l) if l.order == order_id && l.position == position_id => Some(l),
            _ => None,
        })
    }

    /// Get all Location entries
    pub fn locations(&self) -> Vec<&Location> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all Locations for a given order
    pub fn locations_for_order(&self, order: u8) -> Vec<&Location> {
        let order_id = format!("order_{}", order);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.order == order_id => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all Locations for a given position (across all orders)
    pub fn locations_for_position(&self, position: u8) -> Vec<&Location> {
        let position_id = format!("position_{}", position);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.position == position_id => Some(l),
                _ => None,
            })
            .collect()
    }

    // ==========================================================================
    // Systematic Queries - Query semantic/categorical content mapped to anchors
    // ==========================================================================

    // -------------------- Order-Level Systematic Queries --------------------

    /// Get all entries for a given order (everything mapped to that order)
    pub fn system(&self, order: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| e.order() == Some(order))
            .collect()
    }

    /// Get the system name for an order
    pub fn system_name(&self, order: u8) -> Option<&SystemName> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::SystemName(s) if s.order == order_id => Some(s),
            _ => None,
        })
    }

    /// Get the coherence attribute for an order
    pub fn coherence(&self, order: u8) -> Option<&CoherenceAttribute> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::CoherenceAttribute(c) if c.order == order_id => Some(c),
            _ => None,
        })
    }

    /// Get the term designation for an order
    pub fn term_designation(&self, order: u8) -> Option<&TermDesignation> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::TermDesignation(t) if t.order == order_id => Some(t),
            _ => None,
        })
    }

    /// Get the connective designation for an order
    pub fn connective_designation(&self, order: u8) -> Option<&ConnectiveDesignation> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::ConnectiveDesignation(c) if c.order == order_id => Some(c),
            _ => None,
        })
    }

    // -------------------- Location-Level Systematic Queries --------------------

    /// Get all terms for an order, optionally filtered by language of their character
    pub fn terms(&self, order: u8, language: Option<Language>) -> Vec<&Term> {
        let terms: Vec<&Term> = self
            .entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.order_value() == Some(order) => Some(t),
                _ => None,
            })
            .collect();

        if let Some(lang) = language {
            terms
                .into_iter()
                .filter(|t| {
                    self.get_character(&t.character)
                        .map(|c| c.language == lang)
                        .unwrap_or(false)
                })
                .collect()
        } else {
            terms
        }
    }

    /// Get a specific term by order and position
    pub fn term(&self, order: u8, position: u8) -> Option<&Term> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.location == location_id => Some(t),
            _ => None,
        })
    }

    /// Get all terms at a specific location
    pub fn terms_at_location(&self, location_id: &str) -> Vec<&Term> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.location == location_id => Some(t),
                _ => None,
            })
            .collect()
    }

    /// Get all coordinates for an order
    pub fn coordinates(&self, order: u8) -> Vec<&Coordinate> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Coordinate(c) if c.order_value() == Some(order) => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific coordinate by order and position
    pub fn coordinate(&self, order: u8, position: u8) -> Option<&Coordinate> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Coordinate(c) if c.location == location_id => Some(c),
            _ => None,
        })
    }

    /// Get all colours for an order
    pub fn colours(&self, order: u8) -> Vec<&Colour> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Colour(c) if c.order_value() == Some(order) => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific colour by order, position, and language
    pub fn colour(&self, order: u8, position: u8, language: Language) -> Option<&Colour> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Colour(c) if c.location == location_id && c.language == language => Some(c),
            _ => None,
        })
    }

    // -------------------- Character Queries --------------------

    /// Get all characters for a language
    pub fn characters(&self, language: Language) -> Vec<&Character> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Character(c) if c.language == language => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a character by ID
    pub fn get_character(&self, id: &str) -> Option<&Character> {
        self.entries.iter().find_map(|e| match e {
            Entry::Character(c) if c.id == id => Some(c),
            _ => None,
        })
    }

    // -------------------- Cross-Cutting Systematic Queries --------------------

    /// Get all entries at a specific order+position (the "slice" / fiber)
    pub fn slice(&self, order: u8, position: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| e.order() == Some(order) && e.position() == Some(position))
            .collect()
    }

    /// Get all terms at the same position across different languages
    pub fn isomorphic_terms(&self, order: u8, position: u8) -> Vec<(&Term, &Character)> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.location == location_id => {
                    self.get_character(&t.character).map(|c| (t, c))
                }
                _ => None,
            })
            .collect()
    }

    // ==========================================================================
    // Link Queries
    // ==========================================================================

    /// Get connective links, optionally filtered by order and/or base/target positions
    pub fn connectives(
        &self,
        order: u8,
        base_position: Option<u8>,
        target_position: Option<u8>,
    ) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !l.is_connective() {
                    return false;
                }

                // Get the terms for base and target using helper methods
                let base_id = match l.base_single() {
                    Some(id) => id,
                    None => return false,
                };
                let target_id = match l.target_single() {
                    Some(id) => id,
                    None => return false,
                };

                let base_term = self.entries.iter().find_map(|e| match e {
                    Entry::Term(t) if t.id == base_id => Some(t),
                    _ => None,
                });
                let target_term = self.entries.iter().find_map(|e| match e {
                    Entry::Term(t) if t.id == target_id => Some(t),
                    _ => None,
                });

                // Both terms must exist and be in the specified order
                match (base_term, target_term) {
                    (Some(bt), Some(tt))
                        if bt.order_value() == Some(order) && tt.order_value() == Some(order) =>
                    {
                        let base_match = base_position
                            .map(|p| bt.position_value() == Some(p))
                            .unwrap_or(true);
                        let target_match = target_position
                            .map(|p| tt.position_value() == Some(p))
                            .unwrap_or(true);
                        base_match && target_match
                    }
                    _ => false,
                }
            })
            .collect()
    }

    /// Get all connectives involving a specific term
    pub fn connectives_for_term(&self, term_id: &str) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                l.is_connective()
                    && (l.base_single() == Some(term_id) || l.target_single() == Some(term_id))
            })
            .collect()
    }

    /// Get all line links for an order
    pub fn lines(&self, order: u8) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !matches!(l.link_type, LinkType::Line) {
                    return false;
                }

                // Check that base coordinate is in the specified order
                let base_id = match l.base_single() {
                    Some(id) => id,
                    None => return false,
                };

                self.entries.iter().any(|e| match e {
                    Entry::Coordinate(c) if c.id == base_id => c.order_value() == Some(order),
                    _ => false,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entries::Point3d;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();

        // Add anchor entries
        graph.add_entry(Entry::Order(Order::new(3)));
        graph.add_entry(Entry::Position(Position::new(1)));
        graph.add_entry(Entry::Position(Position::new(2)));
        graph.add_entry(Entry::Position(Position::new(3)));
        graph.add_entry(Entry::Location(Location::new(3, 1)));
        graph.add_entry(Entry::Location(Location::new(3, 2)));
        graph.add_entry(Entry::Location(Location::new(3, 3)));

        // Add order-level metadata
        graph.add_entry(Entry::SystemName(SystemName::with_auto_id(3, "Triad")));
        graph.add_entry(Entry::CoherenceAttribute(CoherenceAttribute::with_auto_id(
            3, "Dynamism",
        )));
        graph.add_entry(Entry::TermDesignation(TermDesignation::with_auto_id(
            3, "Impulses",
        )));
        graph.add_entry(Entry::ConnectiveDesignation(
            ConnectiveDesignation::with_auto_id(3, "Acts"),
        ));

        // Add characters
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Will",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Function",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Being",
        )));

        // Add terms (reference locations)
        graph.add_entry(Entry::Term(Term::with_auto_id(3, 1, "char_canonical_will")));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            2,
            "char_canonical_function",
        )));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            3,
            "char_canonical_being",
        )));

        // Add coordinates (reference locations)
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            1,
            Point3d::new(0.0, 1.0, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            2,
            Point3d::new(-0.866, -0.5, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            3,
            Point3d::new(0.866, -0.5, 0.0),
        )));

        // Add colours (reference locations)
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            1,
            Language::Hex,
            "#FF0000",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            2,
            Language::Hex,
            "#0000FF",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            3,
            Language::Hex,
            "#E6E600",
        )));

        graph
    }

    #[test]
    fn test_anchor_queries() {
        let graph = create_test_graph();

        // Order queries
        let order = graph.order(3);
        assert!(order.is_some());
        assert_eq!(order.unwrap().value, 3);

        // Position queries
        let position = graph.position(1);
        assert!(position.is_some());
        assert_eq!(position.unwrap().value, 1);

        // Location queries
        let location = graph.location(3, 1);
        assert!(location.is_some());
        assert_eq!(location.unwrap().id, "loc_3_1");

        // Locations for order
        let locs = graph.locations_for_order(3);
        assert_eq!(locs.len(), 3);

        // Locations for position (cross-order)
        let locs = graph.locations_for_position(1);
        assert_eq!(locs.len(), 1); // Only one order in test graph
    }

    #[test]
    fn test_system_queries() {
        let graph = create_test_graph();

        assert!(graph.system_name(3).is_some());
        assert_eq!(graph.system_name(3).unwrap().value, "Triad");

        assert!(graph.coherence(3).is_some());
        assert_eq!(graph.coherence(3).unwrap().value, "Dynamism");

        assert!(graph.term_designation(3).is_some());
        assert_eq!(graph.term_designation(3).unwrap().value, "Impulses");
    }

    #[test]
    fn test_term_queries() {
        let graph = create_test_graph();

        let terms = graph.terms(3, None);
        assert_eq!(terms.len(), 3);

        let term = graph.term(3, 1);
        assert!(term.is_some());
        assert_eq!(term.unwrap().character, "char_canonical_will");

        // Terms at location
        let terms = graph.terms_at_location("loc_3_1");
        assert_eq!(terms.len(), 1);
    }

    #[test]
    fn test_slice_query() {
        let graph = create_test_graph();

        let slice = graph.slice(3, 1);
        // Should contain: Location, Term, Coordinate, Colour at position 1
        assert_eq!(slice.len(), 4);
    }

    #[test]
    fn test_character_lookup() {
        let graph = create_test_graph();

        let char = graph.get_character("char_canonical_will");
        assert!(char.is_some());
        assert_eq!(char.unwrap().value, "Will");
    }

    #[test]
    fn test_isomorphic_terms() {
        let graph = create_test_graph();

        let iso = graph.isomorphic_terms(3, 1);
        assert_eq!(iso.len(), 1);
        assert_eq!(iso[0].1.value, "Will");
    }
}
