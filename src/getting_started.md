# Getting Started

This section provides simple examples to get you started with `kyori-component-json`.

## Basic text components

The most fundamental way to create a component is with plain text.

```rust
use kyori_component_json::Component;
use serde_json;

fn main() {
    let component = Component::text("Hello, World!");
    let json_output = serde_json::to_string_pretty(&component).unwrap();
    println!("{json_output}");
    // Output:
    // {
    //   "text": "Hello, World!"
    // }
}
```

## Colors and decorations

You can easily apply colors and text decorations (like bold, italic, underlined, etc.) to your components.

```rust
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};
use serde_json;

fn main() {
    let colorful_component = Component::text("Something colorful")
        .color(Some(Color::Named(NamedColor::Red)))
        .decoration(TextDecoration::Bold, Some(true))
        .decoration(TextDecoration::Italic, Some(true));

    let json_output = serde_json::to_string_pretty(&colorful_component).unwrap();
    println!("{json_output}");
    // Output:
    // {
    //   "text": "Something colorful",
    //   "color": "red",
    //   "bold": true,
    //   "italic": true
    // }
}
```

## The `component!()` macro

Since the component builder can be verbose for complex components, there is a component!() macro to simplify it in a declarative way. Here’s a short example to get you started:

```rust
let component = component!(text: "Hello World");
```

This creates a simple hello world component with no formatting. You can find more in the [Rust docs](https://docs.rs/kyori-component-json).