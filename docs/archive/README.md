# Systematics GraphQL API

A GraphQL API for querying systematic data structures organized across multiple dimensions: systems, geometry, topology, vocabulary, metadata, and components.

## Quick Start

### Run the Server

```bash
cargo run --bin api-server
```

The server will start at `http://127.0.0.1:8000/graphql`

### GraphQL Playground

Navigate to `http://127.0.0.1:8000/graphql` in your browser to access the interactive GraphQL Playground where you can explore the schema and run queries.

## Example Queries

### Get All Data for a System

```graphql
query {
  system(name: "octad") {
    name
    coherenceAttribute
    termDesignation
    connectiveDesignation
    source
    termCharacters {
      name
      node
      coordinate { x, y, z }
    }
    connectiveCharacters {
      name
      fromTerm
      toTerm
    }
    points { x, y, z }
    lines {
      start { x, y }
      end { x, y }
    }
    nodes
    edges { from, to }
  }
}
```

### Get Just Geometry for a System

```graphql
query {
  system(name: "tetrad") {
    name
    points { x, y, z }
    lines {
      start { x, y }
      end { x, y }
    }
  }
}
```

Or use the dedicated query:

```graphql
query {
  systemGeometry(name: "tetrad") {
    x
    y
    z
  }
}
```

### Get Just Topology for a System

```graphql
query {
  system(name: "triad") {
    name
    nodes
    edges { from, to }
  }
}
```

### Query by Metadata

**What's the triad's coherence attribute?**

```graphql
query {
  system(name: "triad") {
    coherenceAttribute
  }
}
```

Result: `"Dynamism"`

**What's the tetrad's term designation?**

```graphql
query {
  system(name: "tetrad") {
    termDesignation
    termCharacters {
      name
    }
  }
}
```

Result:
```json
{
  "termDesignation": "Sources",
  "termCharacters": [
    {"name": "Ideal"},
    {"name": "Directive"},
    {"name": "Instrumental"},
    {"name": "Ground"}
  ]
}
```

### Find a Term Across All Systems

**What's the term designation for "Ideal"?**

```graphql
query {
  term(name: "Ideal") {
    name
    systemName
    node
    coordinate { x, y, z }
    system {
      termDesignation
    }
  }
}
```

Result:
```json
{
  "name": "Ideal",
  "systemName": "tetrad",
  "index": 0,
  "coordinate": {"x": 0.0, "y": 1.0, "z": null},
  "system": {
    "termDesignation": "Sources"
  }
}
```

### Get All Systems

```graphql
query {
  allSystems {
    name
    coherenceAttribute
    termDesignation
  }
}
```

### Query Multiple Systems

```graphql
query {
  triad: system(name: "triad") {
    name
    coherenceAttribute
    points { x, y }
  }
  tetrad: system(name: "tetrad") {
    name
    coherenceAttribute
    points { x, y }
  }
}
```

### Find Systems by Attribute

```graphql
query {
  systemsByCoherenceAttribute(attribute: "Dynamism") {
    name
    termDesignation
  }
}
```

### Get Terms in a System

```graphql
query {
  termsInSystem(systemName: "pentad") {
    name
    node
    coordinate { x, y }
  }
}
```

## Available Systems

- **monad** - The first system (Unity)
- **dyad** - Two-element system (Essence, Existence)
- **triad** - Three-element system (Will, Function, Being)
- **tetrad** - Four-element system (Ideal, Directive, Instrumental, Ground)
- **pentad** - Five-element system
- **hexad** - Six-element system
- **heptad** - Seven-element system
- **octad** - Eight-element system
- **ennead** - Nine-element system
- **decad** - Ten-element system
- **undecad** - Eleven-element system
- **dodecad** - Twelve-element system

## GraphQL Schema

### Root Queries

- `system(name: String!)` - Get a complete system by name
- `allSystems` - Get all available systems
- `systemGeometry(name: String!)` - Get just geometry data
- `systemTopology(name: String!)` - Get just topology data
- `systemVocabulary(name: String!)` - Get just vocabulary/terms
- `systemMetadata(name: String!)` - Get just metadata
- `term(name: String!)` - Find a term across all systems
- `termsInSystem(systemName: String!)` - Get all terms in a system
- `systemsByCoherenceAttribute(attribute: String!)` - Find systems by coherence attribute
- `systemsByTermDesignation(designation: String!)` - Find systems by term designation

### Types

**System** - Complete system data
- `name: String!`
- `coherenceAttribute: String!`
- `termDesignation: String!`
- `connectiveDesignation: String!`
- `source: String!`
- `termCharacters: [Term!]!`
- `connectiveCharacters: [Connector!]!`
- `nodes: [Int!]!`
- `edges: [Edge!]!`
- `points: [Coordinate!]!`
- `lines: [Line!]!`

**Term** - A term within a system
- `name: String!`
- `systemName: String!`
- `node: Int!` (the node index in the topology graph)
- `coordinate: Coordinate`
- `system: System`

**Coordinate** - 2D or 3D coordinate
- `x: Float!`
- `y: Float!`
- `z: Float`

**Line** - Connection between two coordinates
- `start: Coordinate!`
- `end: Coordinate!`

**Edge** - Topological edge between nodes
- `from: Int!`
- `to: Int!`

**Connector** - Named connection between terms
- `name: String!`
- `fromTerm: String!`
- `toTerm: String!`

## Development

### Build

```bash
cargo build
```

### Run

```bash
cargo run --bin api-server
```

### Project Structure

```
.
├── Cargo.toml
├── src/
│   ├── core/
│   │   ├── topology.rs       # Topology types (Node, Edge)
│   │   └── geometry.rs       # Geometry types (Coordinates, Point, Line)
│   ├── data/                 # Symlink to ../data
│   ├── graphql/              # GraphQL schema and resolvers
│   ├── lib.rs                # Library entry point
│   └── main.rs               # API server
└── data/                     # Data modules
    ├── by_system/            # Systems (monad, dyad, triad, etc.)
    ├── by_geometry/          # Geometric data (k1-k12)
    ├── by_topology/          # Topological data (k1-k12)
    ├── by_vocabulary/        # Vocabulary data (k1-k12)
    ├── by_component/         # Component data
    └── by_metadata/          # Metadata
```

## CORS

The API has CORS enabled and accepts requests from any origin.

## License

See repository for license information.
