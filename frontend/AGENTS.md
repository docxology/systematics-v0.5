# Frontend Development Guidelines

## Overview

This document provides guidance for AI agents and developers working on the systematics-frontend crate.

## Yew Component Patterns

### Function Components

Prefer function components with hooks for simple UI:

```rust
#[function_component(MyComponent)]
fn my_component(props: &Props) -> Html {
    html! { <div>{&props.value}</div> }
}
```

### Struct Components

Use struct components for complex state management:

```rust
pub struct MyComponent {
    state: SomeState,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;
    
    fn create(ctx: &Context<Self>) -> Self { ... }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool { ... }
    fn view(&self, ctx: &Context<Self>) -> Html { ... }
}
```

### Message Handling

Messages trigger state updates:

```rust
pub enum Msg {
    DataLoaded(Data),
    LoadError(String),
    UserAction(ActionType),
}
```

## GraphQL Client Usage

### Making Queries

```rust
let client = GraphQLClient::new(endpoint);
let result = client.query::<Response>(query_string).await;
```

### Error Handling

Always handle both network and GraphQL errors:

```rust
match result {
    Ok(data) => { /* process */ }
    Err(e) => { /* show error state */ }
}
```

## SVG Rendering Conventions

### Coordinate System

- SVG uses (0,0) at top-left
- Backend provides normalized coordinates
- Apply viewBox and transforms in graph_view.rs

### Element Ordering

1. Edges first (background)
2. Nodes second (foreground)
3. Labels last (topmost)

### Interactive Elements

- Use `onclick` callbacks for navigation
- Style hover states in CSS
- Add `cursor: pointer` for clickable elements

## State Management

### Loading States

Always show loading indicators:

```rust
if self.loading {
    html! { <div class="loading">{"Loading..."}</div> }
} else {
    // render content
}
```

### Error States

Display user-friendly error messages:

```rust
if let Some(error) = &self.error {
    html! { <div class="error">{error}</div> }
}
```

## Testing

### Unit Tests

WASM tests require `wasm-bindgen-test`:

```rust
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_component() { ... }
}
```

### Running Tests

```bash
wasm-pack test --headless --firefox
```

## Build Considerations

### WASM Size

- Avoid large dependencies
- Use `#[wasm_bindgen(start)]` sparingly
- Check bundle size with `twiggy`

### Feature Flags

Middleware uses no server features:

```toml
systematics-middleware = { path = "../middleware" }
# No "server" feature = smaller WASM
```

## Common Tasks

### Add a New Component

1. Create file in `src/components/`
2. Add to `components/mod.rs`
3. Import in parent component
4. Add styles to `styles/main.css`

### Add GraphQL Query

1. Define response types (or use middleware types)
2. Add query method to client
3. Call from component's `create` or `update`
4. Handle loading/error states

### Debug WASM

```rust
web_sys::console::log_1(&"Debug message".into());
```

Or use browser DevTools with source maps.

## Style Guidelines

### CSS Naming

- Use BEM-like naming: `.graph-view__node`
- Component-scoped classes
- CSS variables for colors

### Responsive Design

- Use relative units (em, %)
- Test at multiple viewport sizes
- SVG viewBox handles graph scaling

## File Organization

| File | Responsibility |
|------|---------------|
| `app.rs` | Application state, routing, layout |
| `api/client.rs` | HTTP/GraphQL communication |
| `components/graph_view.rs` | SVG rendering logic |
| `components/system_selector.rs` | Selection UI |
| `styles/main.css` | All styles |

## Performance Tips

- Minimize re-renders (return `false` from `update` when no change)
- Use `key` props for list items
- Batch state updates
- Lazy load large data sets
