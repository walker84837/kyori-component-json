# Introduction to `kyori-component-json`

`kyori-component-json` is an independent, powerful and fluent Rust library designed for working with [Minecraft's JSON text components](https://minecraft.wiki/w/Text_component_format) (Java Edition 1.21.5+).

This system is used and fundamental for displaying and interacting with text in Minecraft. It is used in commands like /tellraw, and in other parts, like books and signs.

## Why is this library named the way it is?

I'll start by breaking down the name in three parts:

- **"kyori"**: The name tries to replicate [Kyori's](https://kyori.net/) Adventure `Component` API, while making it idiomatic in Rust;
- **"component"**: refers to Minecraft's *text component format*;
- **"json"**: The components are parsed (or received via JSON from the [JSON text serializer](https://mvnrepository.com/artifact/net.kyori/adventure-text-serializer-gson)), and deserialized with serde.

Now that the library has switched to [PaperMC](https://github.com/PaperMC/adventure), I'm keeping the same name to avoid causing any disruptions for those who depend on this crate.

## What are components?

In order to understand how to use this library, you should be familiar with Minecraft's text components. These components enable you to send and display rich, interactive text to players.

This format is used in various parts of the game like written books, signs, command messages (e.g., `/tellraw` and `/title`), and other UI elements.

Minecraft still uses JSON text components in places like Adventure, plugins, and commands such as `/tellraw`. These systems expect plain JSON and work the same as before. However, starting in Java Edition 1.21.5, Minecraft now stores text components inside NBT data using SNBT instead of JSON.

This means JSON is still correct for commands and network usage, but components embedded in NBT, such as books, signs, or block/entity data, must be converted to SNBT (in our case, the *server* does this for us).

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

Text components can also be interactive. In **click events**, for example, when a player clicks on certain text, it can trigger various actions:

- `run_command` - Execute a command
- `suggest_command` - Suggest a command in the chat input
- `open_url`- Open a URL in the browser
- `open_file` - Open a file (client-side only)
- `change_page` - Change page in books
- `copy_to_clipboard` - Copy text to the clipboard

Additionally, in **hover events**, when hovering over a text component, a tooltip can appear showing additional information:

- `show_text` - Display another text component with formatting
- `show_item` - Show an item's tooltip with properties like item ID, stack count, and additional components
- `show_entity` - Display entity information including name, entity type ID, and UUID (in either string or integer array format)

These interactive features allow for rich, dynamic text components that can respond to player interactions with various visual feedback and functional actions.

## What can you do with this library?

Now that we understand what a text component is, we have a better understanding of the library's purpose.

This library simplifies the creation, manipulation, and de/serialization of these complex JSON structures, allowing developers to:

* *Create rich text* by generating colorful chat messages with extensive formatting options;
* *Add interactivity* by implementing clickable text that executes commands or opens URLs when a player interacts with it;
* *Provide context* when a player hovers over text, displaying additional information or other text components;
* *Interoperate with Minecraft* clients/servers, making this library necessary for text components.

## Key scenarios

`kyori-component-json` is particularly useful in scenarios such as:

1. *Command generators:* Programmatically create complex `/tellraw`, `/title`, or `/book` commands.
2. *Custom UIs:* Make interactive books, signs, or other text-based interfaces.
3. *Data packs:* Generate dynamic text components for custom advancements, loot tables, or other data pack elements.

[^1]: Within Minecraft they are stored as SNBT, but this library uses JSON. Since `Component` implements serde's Serialize and Deserialize, you can do more than just send components to a server: you can build them, turn them into a string, or parse them back into Components for any purpose. This book will use "JSON" to specifically refer to the JSON representation you use in `/tellraw` and with Adventure's JSON text de/serializer.
