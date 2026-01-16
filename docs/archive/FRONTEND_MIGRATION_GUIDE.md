# Frontend Migration Guide: One-Based Indexing & Node Navigation

## Overview

The systematics-api has been updated with the following major changes:

1. **One-Based Indexing**: All node and edge indices now use one-based indexing (1, 2, 3, ...) instead of zero-based (0, 1, 2, ...)
2. **Semantic Remapping**: Vocabulary orderings have been updated to match semantic meaning
3. **Node Navigation**: New GraphQL fields enable navigation between systems by clicking nodes

## Breaking Changes

### Node and Edge Indexing

**Before (Zero-Based):**
```graphql
{
  system(name: "Triad") {
    nodes  # Returns: [0, 1, 2]
    edges {
      from  # Could be 0, 1, or 2
      to    # Could be 0, 1, or 2
    }
    termCharacters {
      node  # Could be 0, 1, or 2
    }
  }
}
```

**After (One-Based):**
```graphql
{
  system(name: "Triad") {
    nodes  # Returns: [1, 2, 3]
    edges {
      from  # Could be 1, 2, or 3
      to    # Could be 1, 2, or 3
    }
    termCharacters {
      node  # Could be 1, 2, or 3
    }
  }
}
```

### Semantic Vocabulary Changes

The order of vocabulary terms has changed for several systems:

#### Triad
- **Old**: [0=Will, 1=Function, 2=Being]
- **New**: [1=Function, 2=Being, 3=Will]

#### Tetrad
- **Old**: [0=Ideal, 1=Directive, 2=Instrumental, 3=Ground]
- **New**: [1=Ground, 2=Instrumental, 3=Directive, 4=Ideal]

#### Pentad
- **Old**: [0=Purpose, 1=Higher Potential, 2=Quintessence, 3=Lower Potential, 4=Source]
- **New**: [1=Source, 2=Lower Potential, 3=Quintessence, 4=Higher Potential, 5=Purpose]

#### Hexad
- **Old**: [0=Resources, 1=Values, 2=Options, 3=Criteria, 4=Facts, 5=Priorities]
- **New**: [1=Priorities, 2=Facts, 3=Criteria, 4=Options, 5=Values, 6=Resources]

#### Other Systems
Heptad, Octad, Ennead, Dodecad: Vocabulary order unchanged, just +1 indexing offset.
Decad, Undecad: Marked for future semantic research.

## New Features

### 1. Navigation Support

New GraphQL fields enable clicking nodes to navigate between systems:

```graphql
{
  system(name: "Triad") {
    navigationEdges {
      node           # One-based node number (1, 2, or 3)
      targetSystem   # System name ("Monad", "Dyad", or "Triad")
    }
  }
}
```

**Navigation Rule**: Clicking node N navigates to the system with N nodes.
- Node 1 → Monad (1 node)
- Node 2 → Dyad (2 nodes)
- Node 3 → Triad (3 nodes)
- ... and so on up to Node 12 → Dodecad (12 nodes)

### 2. Query Individual Navigation Target

```graphql
{
  system(name: "Hexad") {
    navigationTarget(nodeNumber: 3)  # Returns: "Triad"
  }
}
```

### 3. Enhanced Term Information

The `termCharacters` field now returns `Term` objects with coordinates:

```graphql
{
  system(name: "Triad") {
    termCharacters {
      name          # e.g., "Function"
      node          # One-based: 1, 2, or 3
      systemName    # "Triad"
      coordinate {
        x
        y
        z
      }
    }
  }
}
```

## Migration Steps

### Step 1: Update GraphQL Queries

Replace all queries using the old zero-based indices with one-based equivalents.

**Example Update:**
```javascript
// OLD
const query = `{
  system(name: "Triad") {
    nodes
    termCharacters {
      name
      node
    }
  }
}`;

// Handling OLD response
const handleOldResponse = (data) => {
  data.system.termCharacters.forEach((term) => {
    console.log(`Node ${term.node}: ${term.name}`);
    // Would print: Node 0: Will, Node 1: Function, Node 2: Being
  });
};

// Handling NEW response (same query, different data)
const handleNewResponse = (data) => {
  data.system.termCharacters.forEach((term) => {
    console.log(`Node ${term.node}: ${term.name}`);
    // Now prints: Node 1: Function, Node 2: Being, Node 3: Will
  });
};
```

### Step 2: Update Node Click Handlers

Implement navigation logic for node clicks:

```rust
// In Yew/WASM frontend (ApiGraphView component)
pub enum ApiGraphMsg {
    NodeClicked(usize),        // Array index (still zero-based internally)
    NavigateToSystem(String),  // System name to navigate to
    // ... other messages
}

fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
        ApiGraphMsg::NodeClicked(idx) => {
            // idx is zero-based array index
            // Convert to one-based node number for navigation
            let node_number = (idx + 1) as i32;

            // Find navigation target from preloaded data
            if let Some(nav_edge) = self.navigation_edges.iter()
                .find(|e| e.node == node_number) {
                ctx.link().send_message(
                    ApiGraphMsg::NavigateToSystem(nav_edge.target_system.clone())
                );
            }
            true
        }
        ApiGraphMsg::NavigateToSystem(system_name) => {
            // Update breadcrumb trail
            self.breadcrumbs.push(Breadcrumb {
                system_name: self.current_system.clone(),
                node_clicked: self.selected_node,
            });

            // Load new system
            self.load_system(&system_name);
            true
        }
        // ... other messages
    }
}
```

### Step 3: Add Breadcrumb Trail UI

```rust
struct Breadcrumb {
    system_name: String,
    node_clicked: Option<usize>,
}

fn render_breadcrumbs(&self) -> Html {
    html! {
        <nav class="breadcrumbs">
            { for self.breadcrumbs.iter().map(|(system, node)| {
                let node_label = node.map(|n| format!(" > Node {}", n + 1))
                    .unwrap_or_default();
                html! {
                    <span class="breadcrumb">
                        { system }
                        { node_label }
                        { " > " }
                    </span>
                }
            })}
            <span class="current">{ &self.current_system }</span>
        </nav>
    }
}
```

### Step 4: Update Component State

Update your component to fetch and store navigation edges:

```rust
pub struct ApiGraphView {
    // ... existing fields
    navigation_edges: Vec<NavigationEdge>,
    breadcrumbs: Vec<Breadcrumb>,
    current_system: String,
}

// In GraphQL query
const SYSTEM_QUERY: &str = r#"
    query GetSystem($name: String!) {
        system(name: $name) {
            name
            nodes
            edges { from, to }
            termCharacters {
                name
                node
                coordinate { x, y, z }
            }
            navigationEdges {
                node
                targetSystem
            }
            points { x, y, z }
            lines {
                start { x, y }
                end { x, y }
            }
        }
    }
"#;
```

## Testing Checklist

- [ ] Verify all node labels display as 1-based (1, 2, 3, ..., not 0, 1, 2, ...)
- [ ] Verify Triad shows: 1=Function, 2=Being, 3=Will
- [ ] Verify Tetrad shows: 1=Ground, 2=Instrumental, 3=Directive, 4=Ideal
- [ ] Verify Pentad shows: 1=Source, 2=Lower Potential, 3=Quintessence, 4=Higher Potential, 5=Purpose
- [ ] Verify Hexad shows: 1=Priorities, 2=Facts, 3=Criteria, 4=Options, 5=Values, 6=Resources
- [ ] Test node click navigation:
  - Clicking node 1 in any system navigates to Monad
  - Clicking node 2 in any system navigates to Dyad
  - Clicking node 3 in any system navigates to Triad
  - ... etc.
- [ ] Verify breadcrumb trail updates correctly on navigation
- [ ] Verify geometry renders correctly after vocabulary remapping
- [ ] Test edge connections still display properly with one-based indices

## Troubleshooting

### Issue: Nodes not displaying correctly

**Symptom**: Nodes show as 0, 1, 2 instead of 1, 2, 3
**Cause**: Frontend still using old GraphQL response format
**Solution**: Ensure you're using the updated API and queries

### Issue: Navigation not working

**Symptom**: Clicking nodes doesn't navigate
**Cause**: NavigationEdges not being fetched or used
**Solution**: Add `navigationEdges` to your GraphQL query and implement click handler

### Issue: Wrong terms displayed

**Symptom**: Triad shows Will, Function, Being instead of Function, Being, Will
**Cause**: Frontend caching old data
**Solution**: Clear cache and reload, ensure latest API is being used

## Support

For questions or issues:
- Check the API documentation at `/graphql` playground
- Review the implementation in `src/graphql/types.rs`
- See data files in `data/by_system/` for vocabulary orderings

## API Endpoint

Current API endpoint remains: `http://localhost:8000/graphql`

All changes are backward-compatible in terms of query structure, but the returned data values have changed (indices are now one-based).
