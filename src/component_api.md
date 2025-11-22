## Understanding the `Component` enum

At its core, `kyori-component-json` models Minecraft's text component format using the `Component` enum. This enum represents the three primary ways Minecraft handles text:

- **`Component::String(String)`**: is a simple string, often used as a shorthand for basic text. When serialized, it becomes `{"text": "your string"}`.
- **`Component::Array(Vec<Component>)`**: is a list of components. This is how Minecraft handles messages composed of multiple styled parts, where each part is its own `Component`.
- **`Component::Object(Box<ComponentObject>)`**: is the most extensive form, representing a single text component with all its possible properties. Most of your rich text creation will involve building `ComponentObject`s.

The `ComponentObject` struct:
- holds all the styling and interactive properties a text component can have, such as `color`, and so on;
- contains builder methods like `.color()` or `.decoration()` that internally construct and modify these `ComponentObject` instances.

### Combining multiple components

Complex messages can also be built by appending multiple `Component`s together.

In `Component`, the `append()` method allows you to chain components, and `append_newline()` or `append_space()` add common separators.

When chaining components using `append()`, `append_newline()`, or `append_space()`, styling is inherited from the parent component. However, any explicit styling (e.g., `color`, `decoration`) applied to a child component will override the inherited style for that specific child and its subsequent children, unless they, in turn, explicitly override it. Think of it as a cascading style sheet for your Minecraft text.

We'll use the low-level Component API to make this component.

```rust
use kyori_component_json::{Component, ClickEvent, Color, NamedColor, TextDecoration};
use serde_json;

fn main() {
    let welcome_message = Component::text("Welcome, ")
        .color(Some(Color::Named(NamedColor::White))) // Base color for the first part
        .append(
            Component::text("PlayerName")
                .color(Some(Color::Named(NamedColor::Gold))) // Player name in gold
                .decoration(TextDecoration::Bold, Some(true)) // Player name is bold
        )
        .append(Component::text(" to the server!")) // Appends more text
        .append_newline() // Adds a newline character as a separate component
        .append(
            Component::text("Don\'t forget to read the ")
                .color(Some(Color::Named(NamedColor::Gray))) // Gray text
        )
        .append(
            Component::text("rules")
                .color(Some(Color::Named(NamedColor::LightPurple))) // Rules in light purple
                .decoration(TextDecoration::Underlined, Some(true)) // Rules are underlined
                .click_event(Some(ClickEvent::OpenUrl {
                    url: "https://example.com/rules".to_string(), // Click to open URL
                }))
        )
        .append(Component::text(".")); // Final punctuation

    let json_output = serde_json::to_string_pretty(&welcome_message).unwrap();
    println!("{}", json_output);
}
```
