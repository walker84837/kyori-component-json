# kyori-component-json

> Minecraft text formatting made easy

[![Crates.io](https://img.shields.io/crates/v/kyori-component-json.svg)](https://crates.io/crates/kyori-component-json)
[![Docs.rs](https://docs.rs/kyori-component-json/badge.svg)](https://docs.rs/kyori-component-json)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://github.com/walker84837/kyori-component-json)

A simple Rust library for creating and working with Minecraft's JSON text components (Java Edition 1.21.5+).

These JSON components are used by commands such as `/tellraw` and can be converted as needed for SNBT-based content like books and signs.

## What can you do with this?

Mainly:
- Create tooling around Minecraft clients and [servers](https://jd.advntr.dev/text-serializer-json/4.23.0) easily
- Easier interop between Rust objects and Minecraft's text component format

But it also empowers you to:
- Create colorful chat messages with formatting (bold, italic, etc.)
- Add clickable text that runs commands when clicked
- Make hover text that shows extra information

We also have an [mdBook page](https://walker84837.github.io/kyori-component-json/) for more detailed information on how to use this library.

### When would you use this?

1. **Command Generators**: Create `/tellraw` or `/title` commands programmatically
2. **Custom UIs**: Make interactive books or signs
3. **Data Packs**: Generate dynamic text components

## Simple Examples

### Basic Colored Text

```rust
use kyori_component_json::*;

let message = Component::text("Hello Minecraft!")
    .color(Some(Color::Named(NamedColor::Red)))
    .decoration(TextDecoration::Bold, Some(true));

// Use in /tellraw command:
// /tellraw @a {"text":"Hello Minecraft!","color":"red","bold":true}
```

### Clickable Text

```rust
let clickable = Component::text("Click me!")
    .click_event(Some(ClickEvent::RunCommand {
        command: "/say Hello!".into()
    }))
    .hover_event(Some(HoverEvent::ShowText {
        value: Component::text("This will run a command!")
    }));
```

### Combining Components

```rust
let combined = Component::text("Welcome, ")
    .append(
        Component::text("Player")
            .color(Some(Color::Named(NamedColor::Gold)))
    .append_newline()
    .append(clickable); // From previous example
```

## More Advanced Example

```rust
// Create a formatted message with multiple parts
let message = Component::text("Server Notice: ")
    .color(Some(Color::Named(NamedColor::Red)))
    .append(
        Component::text("Important update!")
            .color(Some(Color::Named(NamedColor::Gold)))
    .append_newline()
    .append(
        Component::text("Click for details")
            .click_event(Some(ClickEvent::RunCommand {
                command: "/update".into()
            }))
            .hover_event(Some(HoverEvent::ShowText {
                value: Component::text("Run /update command")
            }))
    );

// Convert to Minecraft JSON
let json = serde_json::to_string(&message).unwrap();
```

### MiniMessage support (Optional)

This library includes experimental support for parsing and serializing MiniMessage strings, a simplified [markup format](https://docs.advntr.dev/minimessage/index.html). This feature is disabled by default.

To enable it, add the `minimessage` feature to your `Cargo.toml`:

```toml
[dependencies]
kyori-component-json = { version = "0.2", features = ["minimessage"] }
```

Once enabled, you can use the `minimessage` module:

```rust
use kyori_component_json::minimessage::MiniMessage;

let component = MiniMessage::parse("<red>Hello</red> <blue>World!</blue>");
// ... use component ...
```


## Learning More

- [Minecraft Wiki: Raw JSON Text](https://minecraft.wiki/w/Raw_JSON_text_format)
- Library documentation: <https://docs.rs/kyori-component-json> or `cargo doc --open --no-deps`

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome! Please feel free to submit a pull request.
