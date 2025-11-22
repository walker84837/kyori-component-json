# Click and Hover Events

In the previous chapter, we saw a basic example of how to make a component interactive. This chapter will explore the full range of actions available for both `click_event` and `hover_event`, allowing you to create rich, interactive experiences for players.

## Click Events

A `click_event` performs an action when a player clicks on a text component. The `action` tag determines what happens, and it's always paired with a `value`.

Here are the available actions:

* `open_url`: Opens a URL in the user's default web browser.
* `run_command`: Executes a command on the server as the player. For security reasons, this is limited to commands that a player has permission to run.
* `suggest_command`: Prefills the player's chat input with a command, which they can then edit and send.
* `change_page`: Used in books to turn to a specific page.
* `copy_to_clipboard`: Copies a string to the player's clipboard.

Here's an example of a text component that runs a command when clicked:

```rust
use kyori_component_json::{component, ClickEvent};

# fn main() {
let clickable_text = component!(
    text: "Click to say hello!",
    click_event: run_command { command: "/say Hello there!".to_string() }
);
# }
```

## Hover Events

A `hover_event` displays a tooltip when a player hovers their mouse over a text component.

Here are the available hover actions:

* `show_text`: Displays another text component as the tooltip. This can be a simple string or a fully-featured component with its own formatting and events.
* `show_item`: Shows the tooltip of an item, as if hovering over it in the inventory. This requires the item's ID, and can optionally include its count and NBT data.
* `show_entity`: Displays information about an entity, including its name, type, and UUID.

Here's an example of a text component that displays a tooltip when hovered over:

```rust
use kyori_component_json::{component, HoverEvent};

# fn main() {
let hoverable_text = component!(
    text: "Hover over me",
    hover_event: show_text { component!(text: "You found me!") }
);
# }
```