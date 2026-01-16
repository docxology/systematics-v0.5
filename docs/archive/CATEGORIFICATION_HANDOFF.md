# Categorification Handoff Document

This document captures the category theory exploration for the systematics-api project, to be continued after repo consolidation.

---

## Immediate Task: Extend Link Struct

Extend the Link struct to support multiple bases/targets, enabling future morphism types.

### Files to Modify

| File | Changes |
|------|---------|
| `src/core/links.rs` | Change `base: String` to `base: Option<Vec<String>>`, change `target: String` to `target: Option<Vec<String>>` |
| `src/core/graph.rs` | Update Link accessors to handle Option<Vec<String>> |
| `src/data/mod.rs` | Update Link construction to use new signatures |
| `src/graphql/types.rs` | Update Link-related resolvers if needed |

### Implementation Steps

1. **Extend Link struct** (`src/core/links.rs`)
   - Change `base: String` to `base: Option<Vec<String>>`
   - Change `target: String` to `target: Option<Vec<String>>`
   - Add helper methods: `base_single()`, `target_single()`, `bases()`, `targets()`

2. **Update Link constructors** (`src/core/links.rs`)
   - Update `Link::new()` to accept `Option<Vec<String>>` for base/target
   - Update `Link::line()` to wrap single values: `Some(vec![base.into()])`
   - Update `Link::connective()` similarly

3. **Update Graph methods** (`src/core/graph.rs`)
   - Update code that accesses `link.base` or `link.target` directly

4. **Update data builder** (`src/data/mod.rs`)
   - Update Link construction calls to match new signatures

5. **Verify**: `cargo build && cargo test`

### Design

```rust
// src/core/links.rs
pub struct Link {
    pub id: String,
    pub base: Option<Vec<String>>,      // Source(s)
    pub target: Option<Vec<String>>,    // Target(s)
    pub link_type: LinkType,
    pub tag: Option<String>,
}

// LinkType remains unchanged
pub enum LinkType {
    Line,                    // Coordinate → Coordinate
    Connective(String),      // Term → Term (via Character ID)
}
```

---
---

# Categorical Structure Discussion

## Goal
Add explicit morphism links to formalize the categorical relationships between entry types.

## Categorical Structure

### Location as Pullback
Location is where Order and Position meet:
```
         Location
          ▲    ▲
         │    │
     Order    Position
```
- Order 3 maps to Locations {3.1, 3.2, 3.3}
- Position 1 maps to Locations {1.1, 2.1, 3.1, ...12.1}
- Location = { (o, p) | p ≤ o } - constrained product (Position ≤ Order)

### Limits at the Order Level
**TermDesignation** and **ConnectiveDesignation** are limits:
- All Terms in an Order → TermDesignation (e.g., "Impulses" for Triad)
- All Connectives in an Order → ConnectiveDesignation (e.g., "Acts" for Triad)

**CoherenceAttribute** is the pullback of TD and CD (where they meet):
```
         CoherenceAttribute
              ▲    ▲
             │    │
    TermDesignation    ConnectiveDesignation
              ▲              ▲
             │              │
         [Terms]      [Connectives]
```
Both TD and CD are aspects unified by the system's coherence.

**SystemName ≅ Order** - isomorphic (bijection between them), kept separate as distinct entry types.

### Initial and Terminal Objects (Open Question)
The direction depends on perspective - this is covariant/contravariant duality:
- **Covariant**: Order → Position → Location (Order determines Positions)
- **Contravariant**: Position → Order → Location (Position exists across Orders)

In the pullback, both Order and Position flow INTO Location. Neither may be strictly initial/terminal - they're both components of the pullback. Systems embed non-additively: Dyad → Triad is structural transformation (man/woman → father/mother/child), not addition.

### Location as Limit of Triad (Order, Position, Coordinate)
Order, Position, and Coordinate form a triad. Location is the limit (apex) that projects onto all three:
```
              Location (limit/apex)
             ╱    │    ╲
            ╱     │     ╲
           ▼      ▼      ▼
        Order ── Position ── Coordinate
     (affirming) (receptive) (reconciling)
```
- **Order**: Affirming - determines which system
- **Position**: Receptive - determines which slot
- **Coordinate**: Reconciling - geometric realization
- **Location**: The limit where all three are triangulated

This forms a tetrahedron: Location at apex, triad at base.

### Slice as Location Query
Slice is a query of type Location that triangulates in a Hilbert space:
- Retrieves Order, Position, Coordinate to build structures in multidimensional space
- Term and Colour map to Location (not part of the triad limit)
- Slice is not a distinct entity - it's a query pattern over Location

### Covering Relationships (Monomorphisms)
- **Character → Term** (Character covers Term)
- **Character → Connective** (Character covers Connective)
- **Term → Location** (Term at Location)
- **Colour → Location** (Colour at Location, like graph coloring)
- **Coordinate**: Part of the triad (Location limit), not a separate covering relationship

## Current State
- **Base Objects**: Order, Position, Location (pullback of Order and Position)
- **Entry Types**: Term, Coordinate, Colour, Character, SystemName, CoherenceAttribute, TermDesignation, ConnectiveDesignation
- **Link Types**: `Line` (Coordinate→Coordinate), `Connective(String)` (Term→Term via Character ID)

## Future: Morphism Taxonomy

Once basic morphisms work, develop a category of morphism types:

- **Monomorphisms**: Injective (covering relationships documented above)
- **Epimorphisms**: Surjective
- **Isomorphisms**: Bijective (SystemName ≅ Order)
- **Bimorphisms**: Both mono and epi (a specific form of isomorphism)
- **Homomorphisms**: Structure-preserving maps between algebraic structures
- **Holomorphisms**: Complex-analytic structure-preserving maps
- **Natural Transformations**: Vocabulary-to-vocabulary functors (Canonical → Energy) that preserve structure

---

## Future: Commuting Square / Composability Engine

Open questions about the interdependencies between Order, Position, Coordinate, and Location:

### Three Aspects of Representation
1. **Topological** (structure/connectivity): Order, Position - simplex-oriented
2. **Geometric** (spatial realization): Coordinate - embeds topology into Euclidean space
3. **Semantic** (meaning): Character, Term, Connective - vocabulary

### Dimension as Functor Parameter
- Dimension modifies the Topological→Geometric mapping
- Same K4 topology → square-with-cross (2D) or tetrahedron (3D)
- Changing Dimension should recompute all Coordinates

### Open Questions
- Are Order and Position the topological objects, with Location as the relation/operation in the category of topology?
- Is Coordinate the geometric realization of Position, and if so, what is Order's geometric counterpart (the shape/simplex itself)?
- How to represent these as linked interdependencies rather than hierarchical layers?
- Commuting square structure:
  ```
      Order ←────────→ Position
         ↑ ╲        ╱ ↑
         │   ╲    ╱   │
         │    Location
         │   ╱    ╲   │
         ↓ ╱        ╲ ↓
     Coordinate ←──→ ???
  ```
- This may require dependent types, pullbacks in slice categories, or operads/multicategories

### Goal: Composability Engine
A system where:
- Morphisms express the interdependencies
- Changing parameters (Dimension, Order) cascades recomputation
- Structure is preserved through functorial mappings
- Approaching Turing completeness via categorical foundations
