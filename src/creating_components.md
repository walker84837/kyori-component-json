# Creating components

Now that we know what problem this library solves, and that we understand what a Minecraft text component is, we're going to create some components!

## Basic colored text

Let's start with something simple. First, we'll display "Hello Minecraft!" in red and bold.

```rust
use kyori_component_json::{Component, Color, NamedColor, TextDecoration};
use serde_json; // Needed for getting the Component as JSON

fn main() {
    let message = Component::text("Hello Minecraft!")
        .color(Some(Color::Named(NamedColor::Red)))
        .decoration(TextDecoration::Bold, Some(true));

    // To use this in Minecraft, you'd serialize it to JSON:
    let json_output = serde_json::to_string(&message).unwrap();
    println!("{}", json_output);
    // Expected output: {"text":"Hello Minecraft!","color":"red","bold":true}
}
```

The output JSON can then be used in a /tellraw command:
```json
/tellraw @a {"text":"Hello Minecraft!","color":"red","bold":true}
```

This basic example demonstrates how easily you can create a `Component` and apply styling. The library handles the conversion to the specific JSON format that Minecraft understands.

## The `component!` macro

Since the component builder can be verbose, the library provides the `component!` macro to simplify creation with a more declarative and readable syntax.

Here's a simple "Hello World" with no formatting:

```rust
use kyori_component_json::component;

# fn main() {
let component = component!(text: "Hello World");
# }
```

### Formatting

You can easily add colors and decorations. Colors can be specified by name (like `red`) or as a hex code (like `#ff5555`).

```rust
use kyori_component_json::{component, Component, Color, NamedColor, TextDecoration};

# fn main() {
let component = component!(text: "This is important!", {
    color: red,
    decoration: bold & true,
});

// This is equivalent to the builder pattern:
let equivalent = Component::text("This is important!")
    .color(Some(Color::Named(NamedColor::Red)))
    .decoration(TextDecoration::Bold, Some(true));

assert_eq!(component, equivalent);
# }
```

### Nesting Components

To create more complex messages, you can nest `component!` macro calls. This is useful for applying different formatting to different parts of your message.

```rust
use kyori_component_json::{component, Component, Color, NamedColor};

# fn main() {
let component = component!(text: "You can mix ", {
    color: gray,
    append: (component!(text: "styles", {
        color: yellow,
        append: (component!(text: "!", {
            color: gray,
        }))
    }))
});
# }
```

### Adding Interactivity with Events

The macro also makes it easy to add `click_event` and `hover_event` to make your components interactive. This provides a first look at creating interactive components; the next chapter will explore all the available event types in detail.

Let's create a message that runs a command when clicked and displays helpful text when hovered over.

```rust
use kyori_component_json::{component, ClickEvent, HoverEvent, Color, NamedColor};

# fn main() {
let interactive_message = component!(
    text: "Click me to teleport!", {
        color: aqua,
        click_event: run_command { command: "/tp @s 100 64 100".to_string() },
        hover_event: show_text {
            component!(
                text: "Teleports you to spawn coordinates (100, 64, 100)", {
                    color: gray
                }
            )
        }
    }
);
# }
```

### Why not just make the builder more fluent?

As it stands, the `Component` enum aims to closely be a Rust representation of a raw JSON text component, with a few functions to simplify creating and handling one at a low level. As such, it's not designed to be any more fluent than it is now.

The `component!()` macro is a more user-friendly way to create components, and it is recommended to use it in most cases.
