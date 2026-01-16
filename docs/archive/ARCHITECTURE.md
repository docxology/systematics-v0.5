# Systematics Framework - Core Architecture

## Overview

This document describes the core architecture of the Systematics framework, built on Category Theory principles. The framework supports **Bidirectional Construction**: both top-down factory creation and bottom-up inference assembly.

## Core Concepts

### 1. The Essence Layer (Universals)

The Essence layer represents universal, unchanging metadata for all systems (1-12).

#### UniversalIndex

- **Location**: `src/core/universal_index.rs`
- **Purpose**: One-based indexing (1-12) for all systems
- **Key Methods**:
  - `new(value: u8)` - Create from one-based value
  - `value()` - Get one-based value
  - `to_zero_based()` - Convert to array index
  - `from_zero_based(index: usize)` - Create from array index

```rust
let index = UniversalIndex::new(3)?; // Triad
assert_eq!(index.value(), 3);
assert_eq!(index.to_zero_based(), 2);
```

#### Essence Registry

- **Location**: `src/core/essence.rs`
- **Purpose**: Static lookup of universal system properties
- **Data Structure**:

```rust
pub struct EssenceData {
    pub system_name: &'static str,           // Monad, Dyad, Triad...
    pub coherence: &'static str,             // Unity, Complementarity...
    pub term_designation: Option<&'static str>,     // Totality, Poles...
    pub connective_designation: Option<&'static str>, // Unity, Force...
}
```

- **Lookup Methods**:
  - `Essence::lookup(index)` - Get full EssenceData
  - `Essence::system_name(index)` - Get system name
  - `Essence::coherence(index)` - Get coherence attribute
  - `Essence::term_designation(index)` - Get term designation (if available)
  - `Essence::connective_designation(index)` - Get connective designation (if available)

**Design Principle**: Essence data is NEVER duplicated. It's stored once in a static registry and looked up on demand.

### 2. The Existence Layer (Particulars)

The Existence layer separates **Topology** (mandatory/structural) from **Content** (variable/swappable).

#### A. Coordinates (Mandatory)

**SystemTopology** provides canonical coordinates for each system order.

- **Location**: `src/core/system_topology.rs`
- **Purpose**: Define the geometric structure of each system
- **Key Type**:

```rust
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

- **Key Methods**:
  - `SystemTopology::get_coordinates(order: u8)` - Get all coordinates for a system
  - `SystemTopology::get_coordinate(index, order)` - Get coordinate for specific index
  - `SystemTopology::get_adjacency_matrix(order)` - Get connectivity matrix

**Example**:
```rust
let coords = SystemTopology::get_coordinates(3)?; // Triad coordinates
// Returns 3 Point3d values arranged as a triangle
```

**Design**: Each system order (1-12) has canonical coordinates:
- Monad: origin point
- Dyad: two points on x-axis
- Triad: equilateral triangle
- Tetrad: square/cross
- Pentad through Dodecad: regular polygons

#### B. Content (Variable)

**SystemContent** trait enables swappable vocabulary/data.

- **Location**: `src/core/system_content.rs`
- **Purpose**: Generic trait for any content type

```rust
pub trait SystemContent: Clone + Debug {
    fn provide(&self, count: usize) -> Vec<Self>;
    fn display_name(&self) -> String;
}
```

**Built-in Implementations**:
- `String` - Generic text content
- `NumberedItem` - Generic numbered items

**Note**: Colors are handled by `HyparchicRegistry` (monadic adjunction: Index ↔ Color), not SystemContent.

**Example**:
```rust
// String implementation
let base = String::from("Level");
let items = base.provide(5);
// Returns: ["Level 1", "Level 2", "Level 3", "Level 4", "Level 5"]

// NumberedItem implementation
let item = NumberedItem { label: "Step".to_string(), number: 0 };
let items = item.provide(3);
// Returns: [Step 1, Step 2, Step 3]
```

### 3. The Fiber (The Unit)

The **Fiber** is the fundamental lightweight unit combining index, coordinates, and content.

- **Location**: `src/core/fiber.rs`
- **Structure**:

```rust
pub struct Fiber<T: SystemContent> {
    index: UniversalIndex,      // Which position (1-12)
    coordinates: Point3d,        // Where it's located
    content: T,                  // What it contains
}
```

- **Key Methods**:
  - `new(index, coordinates, content)` - Create a fiber
  - `index()` - Get the UniversalIndex
  - `coordinates()` - Get the Point3d
  - `content()` - Get reference to content
  - `content_mut()` - Get mutable content
  - `into_content()` - Take ownership of content

**Essence Lookups** (on-the-fly, no storage):
  - `system_name()` - Lookup from Essence layer
  - `coherence()` - Lookup from Essence layer
  - `term_designation()` - Lookup from Essence layer
  - `connective_designation()` - Lookup from Essence layer
  - `essence()` - Get full EssenceData

**Example**:
```rust
let fiber = Fiber::new(
    UniversalIndex::new(1)?,
    Point3d::new(0.0, 0.0, 0.0),
    String::from("Item")
);

assert_eq!(fiber.system_name(), "Monad");  // Looked up, not stored!
assert_eq!(fiber.content(), &String::from("Item"));
```

### 4. The System (The Container)

The **System<T>** holds a collection of fibers with connectivity.

- **Location**: `src/core/system.rs`
- **Structure**:

```rust
pub struct System<T: SystemContent> {
    fibers: Vec<Fiber<T>>,
    connectivity: Vec<Vec<bool>>,  // Adjacency matrix
}
```

- **Query Methods**:
  - `order()` - Get system order (fiber count)
  - `fibers()` - Get all fibers
  - `connectivity()` - Get adjacency matrix
  - `is_connected(from, to)` - Check if two indices are connected
  - `connections(index)` - Get all connections for an index

## 5. Bidirectional Construction

### Method A: Top-Down Factory

**Factory Pattern**: "Create a Triad using Colors"

```rust
pub fn generate(order: u8, content_provider: T) -> Result<System<T>, SystemError>
```

**Process**:
1. Validate order (1-12)
2. Get canonical coordinates from SystemTopology
3. Request `order` content items from provider
4. Create fibers by combining indices, coordinates, and content
5. Generate connectivity matrix (complete graph)
6. Return assembled System

**Example**:
```rust
let triad = System::generate(3, String::from("Item"))?;
// Creates a 3-node system with Red, Green, Blue content
// Automatically gets correct triangle coordinates
// Automatically generates complete graph connectivity
```

**Use Cases**:
- Generating new systems programmatically
- Creating template systems
- Factory-based construction

### Method B: Bottom-Up Assembly (Inference)

**Inference Pattern**: "Here are components; validate and assemble"

```rust
pub fn try_assemble(fibers: Vec<Fiber<T>>) -> Result<System<T>, SystemError>
```

**Process**:
1. Count fibers to infer system order
2. Validate indices are correct sequence (1, 2, 3, ..., order)
3. Validate coordinates match canonical topology
4. Sort fibers by index
5. Infer/generate connectivity matrix
6. Return validated System or error

**Example**:
```rust
let coords = SystemTopology::get_coordinates(2)?;

let fiber1 = Fiber::new(
    UniversalIndex::new(1)?,
    coords[0],
    String::from("Essence")
);

let fiber2 = Fiber::new(
    UniversalIndex::new(2)?,
    coords[1],
    String::from("Existence")
);

let dyad = System::try_assemble(vec![fiber1, fiber2])?;
```

**Validation Rules**:
- Fiber count must be 1-12
- Indices must be sequential (1, 2, 3, ...)
- Coordinates must match canonical topology (within tolerance)
- No duplicate indices

**Custom Connectivity**:
```rust
pub fn try_assemble_with_connectivity(
    fibers: Vec<Fiber<T>>,
    connectivity: Vec<Vec<bool>>
) -> Result<System<T>, SystemError>
```

## Error Handling

```rust
pub enum SystemError {
    InvalidOrder(String),
    InvalidIndices(String),
    InvalidCoordinates(String),
    InvalidFiberCount(String),
    TopologyMismatch(String),
}
```

All construction methods return `Result<System<T>, SystemError>` for comprehensive error handling.

## Module Structure

```
src/core/
├── mod.rs                   # Module exports
├── universal_index.rs       # UniversalIndex (1-12)
├── essence.rs              # Static Essence registry
├── system_topology.rs      # Canonical coordinates & adjacency
├── system_content.rs       # SystemContent trait & implementations
├── fiber.rs                # Fiber<T> unit
└── system.rs               # System<T> container & construction
```

## Design Principles

1. **Separation of Concerns**:
   - Essence (universal) vs Existence (particular)
   - Topology (structure) vs Content (data)

2. **No Duplication**:
   - Essence data stored once, looked up on demand
   - Canonical coordinates defined once per order

3. **Type Safety**:
   - UniversalIndex ensures valid 1-12 range
   - Generic SystemContent allows any data type
   - Result types for error handling

4. **Bidirectional Flexibility**:
   - Top-down: Generate from specification
   - Bottom-up: Validate and assemble from parts

5. **Lightweight Fibers**:
   - Store only essential data (index, coordinates, content)
   - Lookup metadata on demand

## Example Usage

See `examples/bidirectional_construction.rs` for comprehensive examples:

```bash
cargo run --example bidirectional_construction
```

This demonstrates:
- Top-down generation with different content types
- Bottom-up assembly with validation
- Combined patterns (generate → decompose → reassemble)
- Error handling for invalid constructions

## Testing

Run tests with:
```bash
cargo test
```

Each module includes unit tests:
- `universal_index`: Index creation and conversion
- `essence`: Essence lookups
- `system_topology`: Coordinate generation
- `system_content`: Content providers
- `fiber`: Fiber creation and essence lookups
- `system`: Both construction methods and validation
