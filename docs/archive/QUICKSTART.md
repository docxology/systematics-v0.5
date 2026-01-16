# Systematics Core Framework - Quick Start Guide

## Overview

The Systematics framework supports **two ways** to create systems:

1. **Top-Down (Factory)**: "Create a Triad using Colors"
2. **Bottom-Up (Inference)**: "Here are components, validate and assemble"

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
systematics = { path = "../path/to/systematics-api" }
```

Or use it directly in this workspace:
```rust
use systematics::core::*;
```

## Quick Examples

### 1. Top-Down: Generate a System

```rust
use systematics::core::{
    system::System,
    system_content::Color,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Triad (3 elements) with Color content
    let triad = System::generate(3, String::from("Item"))?;

    println!("Created {} with {} fibers",
        triad.fibers()[0].system_name(),
        triad.order()
    );

    // Access fibers
    for fiber in triad.fibers() {
        println!("  Fiber {}: {:?} at ({:.2}, {:.2})",
            fiber.index().value(),
            fiber.content(),
            fiber.coordinates().x,
            fiber.coordinates().y
        );
    }

    Ok(())
}
```

**Output:**
```
Created Monad with 3 fibers
  Fiber 1: Red at (0.00, -1.00)
  Fiber 2: Green at (-0.50, 0.87)
  Fiber 3: Blue at (-0.50, -0.87)
```

### 2. Bottom-Up: Assemble from Components

```rust
use systematics::core::{
    fiber::Fiber,
    system::System,
    system_topology::SystemTopology,
    universal_index::UniversalIndex,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get canonical coordinates for a Dyad (2 elements)
    let coords = SystemTopology::get_coordinates(2)?;

    // Create fibers manually
    let fiber1 = Fiber::new(
        UniversalIndex::new(1)?,
        coords[0],
        "Essence".to_string()
    );

    let fiber2 = Fiber::new(
        UniversalIndex::new(2)?,
        coords[1],
        "Existence".to_string()
    );

    // Assemble and validate
    let dyad = System::try_assemble(vec![fiber1, fiber2])?;

    println!("Assembled {} successfully!", dyad.fibers()[0].system_name());

    Ok(())
}
```

### 3. Using Different Content Types

```rust
use systematics::core::{
    system::System,
    system_content::{Color, AristotelianCause},
};

// With Colors
let triad = System::generate(3, String::from("Item"))?;

// With Aristotelian Causes
let tetrad = System::generate(4, AristotelianCause::Formal)?;

// With Strings
let pentad = System::generate(5, String::from("Level"))?;
```

### 4. Creating Custom Content

```rust
use systematics::core::system_content::SystemContent;

#[derive(Debug, Clone)]
pub enum Element {
    Fire, Water, Earth, Air
}

impl SystemContent for Element {
    fn provide(&self, count: usize) -> Vec<Self> {
        vec![Element::Fire, Element::Water, Element::Earth, Element::Air]
            .into_iter()
            .take(count)
            .collect()
    }
}

// Use it
let tetrad = System::generate(4, Element::Fire)?;
```

### 5. Querying Connectivity

```rust
use systematics::core::{
    system::System,
    system_content::Color,
    universal_index::UniversalIndex,
};

let triad = System::generate(3, String::from("Item"))?;

// Check if two fibers are connected
let idx1 = UniversalIndex::new(1)?;
let idx2 = UniversalIndex::new(2)?;

if triad.is_connected(idx1, idx2)? {
    println!("Fiber 1 and 2 are connected!");
}

// Get all connections for a fiber
let connections = triad.connections(idx1)?;
println!("Fiber 1 is connected to {} other fibers", connections.len());
```

### 6. Accessing Essence Data

```rust
use systematics::core::{
    fiber::Fiber,
    system_topology::SystemTopology,
    universal_index::UniversalIndex,
};

let coords = SystemTopology::get_coordinates(1)?;
let fiber = Fiber::new(
    UniversalIndex::new(1)?,
    coords[0],
    "Unity".to_string()
);

// Essence lookups (no storage, looked up on demand)
println!("System: {}", fiber.system_name());        // "Monad"
println!("Coherence: {}", fiber.coherence());       // "Unity"
println!("Term: {:?}", fiber.term_designation());   // Some("Totality")
```

## Available Systems (1-12)

| Order | Name | Coherence | Term Designation |
|-------|------|-----------|------------------|
| 1 | Monad | Unity | Totality |
| 2 | Dyad | Complementarity | Poles |
| 3 | Triad | Dynamism | Impulses |
| 4 | Tetrad | Activity Field | Sources |
| 5 | Pentad | Significance & Potential | Limits |
| 6 | Hexad | Coalescence | Laws |
| 7 | Heptad | Generation | Characters |
| 8 | Octad | Self Sufficiency | Elements |
| 9 | Ennead | Transformation | None* |
| 10 | Decad | Intrinsic Harmony | None* |
| 11 | Undecad | Articulate Symmetry | None* |
| 12 | Dodecad | Perfection | None* |

*Research needed for indices 9-12

## Running the Examples

```bash
# Comprehensive bidirectional construction examples
cargo run --example bidirectional_construction

# Run all tests
cargo test

# Run specific test module
cargo test core::system
```

## Error Handling

All construction methods return `Result<System<T>, SystemError>`:

```rust
use systematics::core::system::{System, SystemError};

match System::generate(3, String::from("Item")) {
    Ok(system) => println!("Success! Created {}", system.order()),
    Err(SystemError::InvalidOrder(msg)) => println!("Invalid order: {}", msg),
    Err(SystemError::InvalidIndices(msg)) => println!("Invalid indices: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Common Patterns

### Pattern 1: Generate, Modify, Reassemble
```rust
// Generate
let original = System::generate(3, String::from("Item"))?;

// Extract and modify fibers
let mut fibers: Vec<_> = original.fibers().iter().cloned().collect();
// ... modify fibers ...

// Reassemble
let modified = System::try_assemble(fibers)?;
```

### Pattern 2: Validate External Data
```rust
// Receive fibers from external source (API, file, etc.)
let external_fibers = receive_fibers_from_somewhere();

// Validate and assemble
match System::try_assemble(external_fibers) {
    Ok(system) => process_valid_system(system),
    Err(e) => handle_invalid_data(e),
}
```

### Pattern 3: Mixed Content Types
```rust
// Each System can have its own content type
let triad_colors = System::generate(3, String::from("Item"))?;
let tetrad_causes = System::generate(4, AristotelianCause::Formal)?;
let pentad_strings = System::generate(5, String::from("Item"))?;
```

## Key Concepts

- **UniversalIndex**: 1-based index (1-12) with type safety
- **Essence**: Universal metadata looked up on demand (not stored)
- **Topology**: Mandatory canonical coordinates per system order
- **Content**: Swappable, generic data (implement `SystemContent` trait)
- **Fiber**: Lightweight unit (index + coordinates + content)
- **System**: Container with validation and connectivity

## Documentation

- **ARCHITECTURE.md** - Complete architectural documentation
- **CORE_FRAMEWORK_SUMMARY.md** - Implementation summary
- **examples/bidirectional_construction.rs** - Comprehensive examples

## Help

For issues or questions:
1. Check the documentation files above
2. Run the examples with `--help` flag (if supported)
3. Review the test files in `src/core/*/tests.rs`

## Next Steps

After mastering the basics:
1. Create custom `SystemContent` implementations for your domain
2. Explore connectivity patterns beyond complete graphs
3. Integrate with visualization tools
4. Build domain-specific systems (causality, processes, etc.)

Happy building! 🎯
