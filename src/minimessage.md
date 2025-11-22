# Custom parsers and MiniMessage

While `kyori-component-json` does programmatic construction of Minecraft components decently well, you might be in situations where you need to parse text from other formats or serialize components into a more human-readable markup.

In this case, custom parsers and serializers, become a godsend. The MiniMessage format is built into this library with the `minimessage` feature.

## The `ComponentParser` and `ComponentSerializer` traits

The `kyori-component-json` library provides two key traits in its `parsing` module that define the interface for converting between string representations and `Component` objects:

- **`ComponentParser`**: This trait defines a `from_string` method that takes a string input and attempts to convert it into a `Component`.
- **`ComponentSerializer`**: This trait defines a `to_string` method that takes a `Component` and converts it into a string representation.

These traits allow for a flexible and extensible system where you can implement support for any text format you desire.

## Using the built-in MiniMessage parser

We skipped using MiniMessage to build components. The reason is that MiniMessage is a markup format that uses strings. This means errors won't be caught at runtime or compile time. MiniMessage is set to non-strict by default and skips incorrect tags. However, these errors would be caught at runtime if the parser were set to strict mode.

The `kyori-component-json` library includes experimental support for MiniMessage parsing and serialization via the `minimessage` feature.

### Enabling the `minimessage` Feature

To use the MiniMessage functionality, you must enable the `minimessage` feature in your `Cargo.toml`:

```bash
cargo add kyori-component-json --features minimessage
```

### Parsing MiniMessage to `Component`

As MiniMessage is a markup language, we'll see it's even shorter than using `component!()` or the `Component` builder API:

```rust
use kyori_component_json::minimessage::MiniMessage;
use kyori_component_json::{Component, Color, NamedColor};
use serde_json;

fn main() {
    let minimessage_string = "<green>Hello, <blue>MiniMessage</blue>!</green>";

    // Create a MiniMessage parser instance (default configuration)
    let mm_parser = MiniMessage::new();

    // Parse the MiniMessage string into a Component
    let component = mm_parser.parse(minimessage_string).unwrap();

    // You can then serialize this Component to Minecraft JSON
    let json_output = serde_json::to_string_pretty(&component).unwrap();
    println!("Parsed Component as JSON:\n{}", json_output);

    // Or inspect the component directly
    if let Component::Object(obj) = component {
        assert_eq!(obj.color, Some(Color::Named(NamedColor::Green)));
        if let Some(extra) = obj.extra {
            if let Component::Object(child_obj) = &extra[0] {
                assert_eq!(child_obj.text, Some("MiniMessage".to_string()));
                assert_eq!(child_obj.color, Some(Color::Named(NamedColor::Blue)));
            }
        }
    }
}
```

### Serializing `Component` to MiniMessage

You can also convert a component into a MiniMessage string. This is useful when displaying the entire component tree is difficult or impractical. Common examples include user output, logging, and configurations.

```rust
use kyori_component_json::minimessage::MiniMessage;
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};

fn main() {
    let component = Component::text("This is ")
        .color(Some(Color::Named(NamedColor::Red)))
        .append(
            Component::text("bold and italic")
                .decoration(TextDecoration::Bold, Some(true))
                .decoration(TextDecoration::Italic, Some(true))
        )
        .append(Component::text(" text."));

    // Serialize the Component back to a MiniMessage string
    let minimessage_output = MiniMessage::to_string(&component).unwrap();
    println!("Component as MiniMessage: {}", minimessage_output);
    // Expected output: <red>This is <bold><italic>bold and italic</italic></bold> text.</red>
}
```

### Customizing MiniMessage parsing

The `MiniMessageConfig` struct allows you to customize the parsing behavior:

* `strict`: If `true`, parsing will fail if tags are not properly closed.
* `parse_legacy_colors`: If `true`, it will parse legacy Minecraft color codes (e.g., `&a` for green).

```rust
use kyori_component_json::minimessage::{MiniMessage, MiniMessageConfig};
use kyori_component_json::Component;

fn main() {
    let config = MiniMessageConfig {
        strict: false, // Allow unclosed tags
        parse_legacy_colors: true, // Parse & codes
    };
    let mm_custom_parser = MiniMessage::with_config(config);

    let legacy_message = "&aHello &bWorld!";
    let component = mm_custom_parser.parse(legacy_message).unwrap();
    println!("Parsed legacy colors: {:?}", component);

    let non_strict_message = "<red>Unclosed tag";
    let component_non_strict = mm_custom_parser.parse(non_strict_message).unwrap();
    println!("Parsed non-strict: {:?}", component_non_strict);
}
```
