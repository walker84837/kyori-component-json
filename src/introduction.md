# Introduction to `kyori-component-json`

`kyori-component-json` is a powerful and easy-to-use Rust library designed for working with Minecraft's JSON text components (Java Edition 1.21.5+). This system is fundamental to how text is displayed and interacted with in Minecraft, used in commands like `/tellraw`, in books, on signs, and various other in-game elements.

## What can you do with this library?

This library simplifies the creation, manipulation, and serialization of these complex JSON structures, enabling developers to:

*   **Create Rich Text:** Easily generate colorful chat messages with various formatting options (bold, italic, underlined, strikethrough, obfuscated).
*   **Add Interactivity:** Implement clickable text that executes commands or opens URLs when a player interacts with it.
*   **Provide Context:** Create hoverable text that displays additional information or other text components when a player hovers over it.
*   **Tooling and Interoperability:** Facilitate seamless interaction between Rust applications and Minecraft clients/servers, making it ideal for building custom tools, plugins, or data pack generators.

## Key Scenarios

`kyori-component-json` is particularly useful in scenarios such as:

1.  **Command Generators:** Programmatically construct complex `/tellraw`, `/title`, or `/book` commands.
2.  **Custom UIs:** Develop interactive in-game books, signs, or other text-based interfaces.
3.  **Data Packs:** Generate dynamic text components for custom advancements, loot tables, or other data pack elements.

## Quick Start: Basic Colored Text

Let's start with a simple example to display "Hello Minecraft!" in red and bold.

```rust
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};
use serde_json; // Required for serialization

fn main() {
    let message = Component::text("Hello Minecraft!")
        .color(Some(Color::Named(NamedColor::Red)))
        .decoration(TextDecoration::Bold, Some(true));

    // To use this in Minecraft, you'd serialize it to JSON:
    let json_output = serde_json::to_string(&message).unwrap();
    println!("{}", json_output);
    // Expected output: {"text":"Hello Minecraft!","color":"red","bold":true}

    // This JSON can then be used in a /tellraw command:
    // /tellraw @a {"text":"Hello Minecraft!","color":"red","bold":true}
}
```

This basic example demonstrates how easily you can create a `Component` and apply styling. The library handles the conversion to the specific JSON format that Minecraft understands.