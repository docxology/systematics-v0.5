# Testing the Systematics GraphQL API

## Start the Server

```bash
cargo run --bin api-server
```

You should see:
```
GraphQL server running at http://127.0.0.1:8000/graphql
GraphQL playground available at http://127.0.0.1:8000/graphql
```

## Option 1: GraphQL Playground (Recommended)

1. Open your browser to: **http://127.0.0.1:8000/graphql**
2. You'll see an interactive GraphQL Playground
3. Try these queries:

### Example Query 1: Get octad system
```graphql
query {
  system(name: "octad") {
    name
    coherenceAttribute
    termDesignation
    termCharacters {
      name
      node
      coordinate { x, y }
    }
    nodes
    edges { from, to }
  }
}
```

### Example Query 2: Find the term "Ideal"
```graphql
query {
  term(name: "Ideal") {
    name
    systemName
    node
    coordinate { x, y, z }
    system {
      name
      termDesignation
    }
  }
}
```

### Example Query 3: Get all systems
```graphql
query {
  allSystems {
    name
    coherenceAttribute
    termDesignation
  }
}
```

## Option 2: Using cURL

```bash
# Get tetrad system
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ system(name: \"tetrad\") { name coherenceAttribute termCharacters { name } } }"
  }'
```

## Option 3: Using a GraphQL Client

Install a GraphQL client like:
- **Insomnia** - https://insomnia.rest
- **Postman** - https://www.postman.com
- **Altair GraphQL Client** (Browser extension)

Point it to: `http://127.0.0.1:8000/graphql`

## Useful Test Queries

### Get just topology for triad
```graphql
query {
  system(name: "triad") {
    name
    nodes
    edges { from, to }
  }
}
```

### Get just geometry for tetrad
```graphql
query {
  system(name: "tetrad") {
    name
    points { x, y }
    lines {
      start { x, y }
      end { x, y }
    }
  }
}
```

### Find systems with specific coherence attribute
```graphql
query {
  systemsByCoherenceAttribute(attribute: "Dynamism") {
    name
    termDesignation
  }
}
```

### Get all terms in pentad
```graphql
query {
  termsInSystem(systemName: "pentad") {
    name
    node
    coordinate { x, y }
  }
}
```

## Introspection

The GraphQL Playground supports **schema introspection** - you can explore:
- All available queries
- All types and their fields
- Documentation for each field

Just click "DOCS" or "SCHEMA" in the playground sidebar!
