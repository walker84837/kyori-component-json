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
