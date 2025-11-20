# Introduction to `kyori-component-json`

`kyori-component-json` is an independent, powerful and fluent Rust library designed for working with [Minecraft's JSON text components](https://minecraft.wiki/w/Text_component_format) (Java Edition 1.21.5+).

This system is used and fundamental for displaying and interacting with text in Minecraft. It is used in commands like /tellraw, and in other parts, like books and signs.

## Why is this library named the way it is?

I'll start by breaking down the name in three parts:

- **"kyori"**: The name tries to replicate [Kyori's](https://kyori.net/) Adventure `Component` API, while making it idiomatic in Rust;
- **"component"**: refers to Minecraft's *text component format*;
- **"json"**: The components are receieved via JSON (via the [JSON text serializer](https://mvnrepository.com/artifact/net.kyori/adventure-text-serializer-gson)), and deserialized with serde.

Now that the library has switched to [PaperMC](https://github.com/PaperMC/adventure), I'm keeping the same name to avoid causing any disruptions for those who depend on this crate.

## What are components?

In order to understand how to use this library, you should be familiar with Minecraft's text components. These components enable you to send and display rich, interactive text to players.

This format is used in various parts of the game like written books, signs, command messages (e.g., `/tellraw` and `/title`), and other UI elements.

### Structure

Text components (also known as raw JSON text) are made up of different types of content and can include formatting, interactivity, and child components that inherit properties from their parent.

For example, if a text component has a certain color or style (like bold or italic), its children will share those properties unless specifically overridden.

The format of these text components uses structured JSON objects[^1] that can include a wide range of properties like color, font, boldness, italics, and even hover or click events. They are highly customizable, allowing for complex interactions and styling.

For instance, you could make a piece of text:

- bold
- red
- clickable to run a specific command when clicked

all by using a combination of tags and nested components.

### Types

There are several types of content that text components can display:

1. *Plain text*: Just simple text to be shown directly.
2. *Translated text*: Text that is translated based on the player's language setting, useful for supporting multiple languages.
3. *Scoreboard values*: Displays player scores from a scoreboard.
4. *Entity names*: Displays names of entities (like players or mobs) based on a selector (like @a or @p).
5. *Keybinds*: Shows the name of a control button bound to a specific action in the player's control scheme.
6. *NBT values*: Displays data from an entity or block, such as health or custom attributes.

#### Hover events and click events

<!-- TODO: compare this to actual source code docs with ClickEvent and HoverEvent types-->

Text components can also be interactive. In **click events**, for example, when a player clicks on certain text, it can trigger actions like running commands (`run_command`), opening URLs (`open_url`), or copying text to the clipboard (`copy_to_clipboard`).

Additionally, in **hover events**, when hovering over a text component, a tooltip can appear, showing more details like an item's properties (`show_item`) or an entity's name and type (`show_entity`).

## What can you do with this library?

This library simplifies the creation, manipulation, and serialization of these complex JSON structures, enabling developers to:

* **Create Rich Text:** Easily generate colorful chat messages with various formatting options (bold, italic, underlined, strikethrough, obfuscated).
* **Add Interactivity:** Implement clickable text that executes commands or opens URLs when a player interacts with it.
* **Provide Context:** Create hoverable text that displays additional information or other text components when a player hovers over it.
* **Tooling and Interoperability:** Facilitate seamless interaction between Rust applications and Minecraft clients/servers, making it ideal for building custom tools, plugins, or data pack generators.

## Key scenarios

`kyori-component-json` is particularly useful in scenarios such as:

1.  **Command Generators:** Programmatically construct complex `/tellraw`, `/title`, or `/book` commands.
2.  **Custom UIs:** Develop interactive in-game books, signs, or other text-based interfaces.
3.  **Data Packs:** Generate dynamic text components for custom advancements, loot tables, or other data pack elements.

## Quick Start: Basic Colored Text

Let's start with a simple example to display "Hello Minecraft!" in red and bold.

```rust
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};
use serde_json; // Required for getting the Component as Minecraft-compatible JSON

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

## `component!()` macro

Since the component builder can be verbose for complex components, there is a `component!()` macro to simplify it. Here's a short example to get you started:

```rust,no_run
let component = component!(text: "Hello World");
```

This creates a simple hello world component with no formatting. You can find more at the [Rust docs](https://docs.rs/kyori-component-json)

### Why not just make the builder more fluent?

As it stands, the `Component` enum aims to closely be a Rust representation of a raw JSON text component, with a few functions to simplify creating and handling one at a low level.

[^1]: Within Minecraft they are stored as SNBT, but this library uses JSON for it (to be sent to a Paper server, using the JSON text serializer)