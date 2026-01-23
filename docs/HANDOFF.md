# Handoff: Simplex-Anchored Data Model

## Current State

PR #1 (`Joshfairhead/data-refactor`) implements **Option A: Simplex-Anchored Connectives**. All 32 tests pass. The refactor anchors connective links to the simplex structure (Locations) rather than vocabulary decorations (Terms).

---

## Data Model Overview

### Entry Taxonomy

Entries are organised into four layers:

| Layer | Types | Anchored To |
|-------|-------|-------------|
| **Anchor** | Order, Position, Location | Nothing (foundation) |
| **Order-level** | SystemName, CoherenceAttribute, TermDesignation, ConnectiveDesignation | Order |
| **Location-level** | Term, Coordinate, Colour | Location (= Order x Position) |
| **Semantic** | Character | Nothing (reusable vocabulary) |

Location is the **pullback** of Order and Position. All location-level entries reference a Location ID, not raw order/position values.

### Link Types

| Type | Connects | Character Storage |
|------|----------|-------------------|
| `Line` | Coordinate -> Coordinate | N/A |
| `Connective` | Location -> Location | `tag` field |

### Bimorphic Relationships

```
Structural     <->     Vocabulary
----------           ----------
Order         <->      Designation    (order-level semantic)
Position      <->      Character      (position-level value)
Location      <->      Term           (decorated vertex)
Edge          <->      Connective     (decorated edge)
```

The simplex (structural) is the source of truth. Vocabulary decorates it.

---

## What Was Refactored (Option A)

### Before
```rust
LinkType::Connective(String)  // Character ID embedded in enum variant
Link {
    base: vec!["term_3_1"],   // Referenced Terms
    target: vec!["term_3_2"],
    link_type: Connective("char_act1"),
    tag: None,
}
```

### After
```rust
LinkType::Connective          // Unit variant, no embedded data
Link {
    base: vec!["loc_3_1"],    // References Locations (simplex-anchored)
    target: vec!["loc_3_2"],
    link_type: Connective,
    tag: Some("char_act1"),   // Character in tag field
}
```

### New Graph Methods

- `term_at_location(location_id)` - canonical Term at a Location
- `term_character_at(location_id)` - Character of term at Location (for rendering)
- `connectives_for_location(location_id)` - connectives involving a Location
- `connectives_for_term(term_id)` - resolves term to location, then queries

### What This Enables

1. Connectives are structurally invariant (survive vocabulary changes)
2. Term characters can be looked up dynamically at render time
3. `tag` field is used properly for metadata rather than overloading the type system

---

## ID Conventions

| Type | Pattern | Example |
|------|---------|---------|
| Order | `order_{n}` | `order_3` |
| Position | `position_{n}` | `position_1` |
| Location | `loc_{order}_{position}` | `loc_3_1` |
| Term | `term_{order}_{position}` | `term_3_1` |
| Coordinate | `coord_{order}_{position}` | `coord_3_1` |
| Character | `char_{language}_{value}` | `char_canonical_will` |
| Connective | `conn_loc_{o}_{p}_loc_{o}_{p}` | `conn_loc_3_1_loc_3_2` |
| Line | `line_coord_{o}_{p}_coord_{o}_{p}` | `line_coord_3_1_coord_3_2` |

---

## Key Files

| File | Role |
|------|------|
| `backend/src/core/entries.rs` | All Entry types (Order, Position, Location, Term, etc.) |
| `backend/src/core/links.rs` | LinkType enum, Link struct, helpers |
| `backend/src/core/graph.rs` | Graph container with query methods |
| `backend/src/core/language.rs` | Language enum (Canonical, Energy, Values, Society) |
| `backend/src/data/mod.rs` | Data construction for orders 1-12 |
| `backend/src/graphql/types.rs` | GraphQL resolvers and types |
| `frontend/src/` | Yew/WASM frontend with SVG renderer |
| `middleware/src/` | Shared wire types (feature-gated for server/wasm) |

---

## Future Work

### Option B: Derived Connective Labels (Next Natural Step)

Currently connective labels are stored explicitly in the `tag` field. Option B derives them:

- Define semantic rules: `(term_char_A, term_char_B) -> label_char`
- Connective stores no label; derives at render time
- Fully couples connective labels to term characters
- When vocabulary changes, connective labels update automatically

Possible inspiration: https://github.com/DefenderOfBasic/good-and-evil-concepts

### Citation Triad

New entry types for attribution:
- **Source** - origin of information
- **Artefact** - the referenced work
- **Lookup** - specific location within artefact
- Citation anchor links
- Attribution relationships

### Functor Infrastructure

Once patterns stabilise:
- Explicit `Functor` type bundling vocabulary mappings
- A Functor = mapping characters to simplex in one go
- Enables vocabulary switching (Canonical <-> Energy <-> Values <-> Society)
- Natural transformations between vocabularies

### GraphQL/Middleware Integration

- Expose bimorphic relationships in the API
- Vocabulary switching queries
- Connective resolution (Location -> Term -> Character) at the resolver level

### Morphism Taxonomy (from Categorification Handoff)

The Link struct already supports multiple bases/targets. Future morphism types:
- Monomorphisms (covering: Character -> Term)
- Epimorphisms (surjective)
- Isomorphisms (SystemName ~ Order)
- Natural Transformations (vocabulary functors)

See `docs/archive/CATEGORIFICATION_HANDOFF.md` for the full categorical discussion.

---

## Running

```bash
# Backend (GraphQL API + Playground)
cd backend && cargo run
# -> http://127.0.0.1:8000/graphql

# Frontend (Yew/WASM)
cd frontend && trunk serve
# -> http://127.0.0.1:8080
```

---

## Architectural Invariants

1. **Location is the pullback** - always Order x Position, never raw values
2. **Connectives anchor to Locations** - not Terms, not Characters
3. **Characters are vocabulary-neutral** - same type for terms and connective labels
4. **Tag field carries metadata** - not the type system
5. **Entry enum is the sum type** - all entry types live in one flat enum
6. **IDs encode structure** - parseable format carries order/position info
