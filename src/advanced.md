# Advanced component usage and customization

As we learned in the previous chapter, we understand what text components are, and how to use `kyori-component-json` effectively and in a concise way.

So far, it's a pretty simple library, but it gives you more tools to make this library even more flexible!

## Understanding the `Component` enum

At its core, `kyori-component-json` models Minecraft's text component format using the `Component` enum. This enum represents the three primary ways Minecraft handles text:

* **`Component::String(String)`**: is a simple string, often used as a shorthand for basic text. When serialized, it becomes `{"text": "your string"}`.
* **`Component::Array(Vec<Component>)`**: is a list of components. This is how Minecraft handles messages composed of multiple styled parts, where each part is its own `Component`.
* **`Component::Object(Box<ComponentObject>)`**: is the most extensive form, representing a single text component with all its possible properties. Most of your rich text creation will involve building `ComponentObject`s.

The `ComponentObject` struct:
- holds all the styling and interactive properties a text component can have, such as `color`, and so on;
- contains builder methods like `.color()` or `.decoration()` that internally construct and modify these `ComponentObject` instances.

## Building interactive messages

As shown in the previous chapter, Minecraft's text components can be made interactive. You can make cool messages by mixing colors, formats, click and hover events.

### Clickable commands and hover text

Let's create a message that runs a command when clicked and displays extra information when hovered over.

This demonstrates how the ClickEvent and HoverEvent are attached to a component to make it interactive.

```rust
use kyori_component_json::{Component, ClickEvent, HoverEvent, Color, NamedColor};
use serde_json;

fn main() {
    let interactive_message = component!(
        text: "Click me to teleport!", {
            color: aqua,
            click_event: run_command { command: "/tp @s 100 64 100".to_string() },
            hover_event: show_text {
                component!(
                    text: "Teleports you to spawn coordinates (100, 64, 100)", { color: gray }
                )
            }
        }
    )

    // Serialize the component to a JSON string, ready for a /tellraw command
    let json_output = serde_json::to_string_pretty(&interactive_message).unwrap();
    println!("{json_output}");

    // This JSON can be used directly in Minecraft:
    // /tellraw @a {"text":"Click me to teleport!","color":"aqua","clickEvent":{...},"hoverEvent":{...}}
}
```

This is a good example where `component!()` can be almost fundamental to creating advanced components concisely.

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

## Displaying dynamic data

Minecraft's text components can also display dynamic in-game information, which updates automatically. `kyori-component-json` provides specific component types for this.

### Scoreboard values

The `Component::score()` constructor allows you to embed a player's score from a specific objective directly into a message. The game client will then resolve and display the current score.

```rust
use kyori_component_json::{Component, ScoreContent, Color, NamedColor};
use serde_json;

fn main() {
    let score_display = Component::text("Your current score: ")
        .color(Some(Color::Named(NamedColor::Green)))
        .append(
            Component::score(ScoreContent {
                name: "@p".to_string(), // Target selector (e.g., @p for nearest player)
                objective: "my_objective".to_string(), // The name of the scoreboard objective
            })
            .color(Some(Color::Named(NamedColor::Yellow))) // Style the score value
        )
        .append(Component::text(" points."));

    let json_output = serde_json::to_string_pretty(&score_display).unwrap();
    println!("{}", json_output);
    // This JSON can be used directly in Minecraft:
    // /tellraw @a {"text":"Your current score: ","color":"green","extra":[{"score":{"name":"@p","objective":"my_objective"},"color":"yellow"}, {"text":" points."}]}
}
```

### Keybinds

To show players controls, you can use `Component::keybind()` to display the current key assigned to a specific action. The Minecraft client will automatically show the player's configured key.

```rust
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};
use serde_json;

fn main() {
    let keybind_message = Component::text("Press ")
        .color(Some(Color::Named(NamedColor::Gray)))
        .append(
            Component::keybind("key.attack") // Minecraft's internal keybind ID (e.g., "key.attack", etc.)
                .color(Some(Color::Named(NamedColor::Red)))
                .decoration(TextDecoration::Bold, Some(true))
        )
        .append(Component::text(" to attack."));

    let json_output = serde_json::to_string_pretty(&keybind_message).unwrap();
    println!("{}", json_output);
    // This JSON can be used directly in Minecraft:
    // /tellraw @a {"text":"Press ","color":"gray","extra":[{"keybind":"key.attack","color":"red","bold":true},{"text":" to attack."}]}
}
```

## Working directly with JSON: serialization and deserialization

One of the core purpose of `kyori-component-json` is to simplify the creation of `Component` structures that can be easily converted to and from JSON. This conversion is done seamlessly by `serde_json`, but you could convert a `Component` to any other format (like CBOR or YAML).

```rust
use kyori_component_json::{Component, Color, NamedColor};
use serde_json;

fn main() {
    let component = Component::text("Hello, JSON!")
        .color(Some(Color::Named(NamedColor::Green)));

    // Serialize to a compact JSON string
    let compact_json = serde_json::to_string(&component).unwrap();
    println!("Compact JSON: {}", compact_json);
    // Output: Compact JSON: {"text":"Hello, JSON!","color":"green"}

    // Serialize to a pretty-printed JSON string for readability
    let pretty_json = serde_json::to_string_pretty(&component).unwrap();
    println!("Pretty JSON:\n{}", pretty_json);

    // Deserialize from a JSON string back into a Component
    let deserialized_component: Component = serde_json::from_str(&compact_json).unwrap();
    assert_eq!(component, deserialized_component);
    println!("Deserialized component matches original: {}", component == deserialized_component);

    // You can also deserialize more complex JSON structures
    let complex_json = r#"#
    {
      "text": "Part 1 ",
      "color": "blue",
      "extra": [
        {"text": "Part 2", "bold": true},
        {"text": " Part 3", "italic": true}
      ]
    }
    "#;
    let complex_component: Component = serde_json::from_str(complex_json).unwrap();
    println!("\nDeserialized complex component: {:?}", complex_component);
}
```

# Custom parsers and MiniMessage

While `kyori-component-json` excels at programmatic construction of Minecraft components, you might encounter situations where you need to parse text from other formats or serialize components into a more human-readable markup. This is where custom parsers and serializers, exemplified by the built-in MiniMessage feature, become invaluable.

## What is MiniMessage?

MiniMessage is a lightweight, human-friendly markup format designed for rich text. Instead of writing verbose JSON, you can use simple tags to apply colors, formatting, and even interactive events. For example, `<red>Hello, <b>World!</b></red>` is much easier to write and read than its JSON equivalent.

MiniMessage is particularly useful when:

*   **User Input:** Allowing players or server administrators to easily format messages without needing to understand complex JSON.
*   **Configuration Files:** Storing rich text in a more readable format within configuration files.
*   **Simplified Development:** Quickly prototyping messages without extensive Rust code.

## The `ComponentParser` and `ComponentSerializer` Traits

The `kyori-component-json` library provides two key traits in its `parsing` module that define the interface for converting between string representations and `Component` objects:

*   **`ComponentParser`**: This trait defines a `from_string` method that takes a string input and attempts to convert it into a `Component`.
*   **`ComponentSerializer`**: This trait defines a `to_string` method that takes a `Component` and converts it into a string representation.

These traits allow for a flexible and extensible system where you can implement support for any text format you desire.

## Using the Built-in MiniMessage Parser

The `kyori-component-json` library includes experimental support for MiniMessage parsing and serialization via the `minimessage` feature.

### Enabling the `minimessage` Feature

To use the MiniMessage functionality, you must enable the `minimessage` feature in your `Cargo.toml`:

```toml
[dependencies]
kyori-component-json = { version = "0.2", features = ["minimessage"] }
serde_json = "1.0" # Required for general JSON serialization/deserialization
```

### Parsing MiniMessage to `Component`

Once enabled, you can use the `MiniMessage` struct to parse MiniMessage strings into `Component` objects.

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

You can also convert a `Component` back into a MiniMessage string.

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

### Customizing MiniMessage Parsing

The `MiniMessageConfig` struct allows you to customize the parsing behavior:

*   `strict`: If `true`, parsing will fail if tags are not properly closed.
*   `parse_legacy_colors`: If `true`, it will parse legacy Minecraft color codes (e.g., `&a` for green).

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

## Implementing your own custom parser

The `ComponentParser` and `ComponentSerializer` traits are designed for extensibility. In case you need to support a different markup format (e.g., a custom Markdown-like syntax, or integration with another rich text system), these traits come in handy.

At a high level:
- The `ComponentParser` trait defines how to convert a string representation of a text component into a `Component` enum. Its primary method is

  ```rust
  fn from_string(input: impl AsRef<str>) -> Result<Component, Self::Err>;
  ```

- The `ComponentSerializer` trait defines how to convert a `Component` enum back into a string representation. Its primary method is:
  ```rust
  fn to_string(component: &Component) -> Result<String, Self::Err>;
  ```

1.  **Define your parser/serializer struct**: Create a new struct (like `MyCustomParser`).
2.  **Implement `ComponentParser`**:
    * Implement the `from_string` method, which will contain your logic to parse the input string and construct a `Component` object. This often involves tokenizing the input, managing a style stack, and building `Component`s based on your format's rules.
3.  **Implement `ComponentSerializer` (optional)**:
    * Implement the `to_string` method, which will traverse a `Component` object and convert it into your desired string format.

Both traits have an `Err` associated type for your serialization errors. You have to define one, but it can be any type, including `()` as a placeholder.

The [`minimessage.rs`](https://github.com/walker84837/kyori-component-json/blob/main/src/minimessage.rs) source file is an excellent reference for how to implement these traits. By following this pattern, you can integrate `kyori-component-json` with virtually any rich text input or output format.

You can find more about these traits on docs.rs:
- [`ComponentParser`](https://docs.rs/kyori-component-json/latest/kyori_component_json/parsing/trait.ComponentParser.html)
- [`ComponentSerializer`](https://docs.rs/kyori-component-json/latest/kyori_component_json/parsing/trait.ComponentSerializer.html)
