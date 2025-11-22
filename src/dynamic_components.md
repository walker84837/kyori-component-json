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
    // /tellraw @a {"text":"Your current score: ","color":"green","extra":[{"score":{...},"color":"yellow"}, {"text":" points."}]}
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