//! # kyori-component-json
//!
//! A library for serialising and deserialising Minecraft's JSON component format, also known as
//! 'raw JSON text'. Minecraft uses this format to display rich text throughout the game, including in
//! commands such as /tellraw and in elements such as books, signs, scoreboards and entity names.
//!
//! ## Features
//! - Full support for Minecraft's component specification (as of Java Edition 1.21.5+)
//! - Serialization and deserialization using Serde
//! - Builder-style API for constructing components
//! - Style inheritance and component nesting
//! - Comprehensive type safety for all component elements
//!
//! ## When to Use
//! This library is useful when:
//! - Generating complex chat messages with formatting and interactivity
//! - Creating custom books or signs with rich text
//! - Building command generators that use `/tellraw` or `/title`
//! - Processing component data from Minecraft APIs or data packs
//!
//! ## Getting Started
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! kyori-component-json = "0.1"
//! serde-json = "1.0"
//! ```
//!
//! ## Basic Example
//! ```
//! use kyori_component_json::*;
//! use serde_json::json;
//!
//! // Create a formatted chat message
//! let message = Component::text("Hello ")
//!     .color(Some(Color::Named(NamedColor::Yellow)))
//!     .append(
//!         Component::text("World!")
//!             .color(Some(Color::Named(NamedColor::White)))
//!             .decoration(TextDecoration::Bold, Some(true))
//!     )
//!     .append_newline()
//!     .append(
//!         Component::text("Click here")
//!             .click_event(Some(ClickEvent::RunCommand {
//!                 command: "/say I was clicked!".into()
//!             }))
//!             .hover_event(Some(HoverEvent::ShowText {
//!                 value: Component::text("Run a command!")
//!             }))
//!     );
//!
//! // Serialize to JSON
//! let json = serde_json::to_value(&message).unwrap();
//! assert_eq!(json, json!({
//!     "text": "Hello ",
//!     "color": "yellow",
//!     "extra": [
//!         {
//!             "text": "World!",
//!             "color": "white",
//!             "bold": true
//!         },
//!         {"text": "\n"},
//!         {
//!             "text": "Click here",
//!             "click_event": {
//!                 "action": "run_command",
//!                 "command": "/say I was clicked!"
//!             },
//!             "hover_event": {
//!                 "action": "show_text",
//!                 "value": {"text": "Run a command!"}
//!             }
//!         }
//!     ]
//! }));
//! ```
//!
//! ## Key Concepts
//! 1. **Components** - The building blocks of Minecraft text:
//!    - `String`: Plain text shorthand
//!    - `Array`: List of components
//!    - `Object`: Full component with properties
//! 2. **Content Types** - Special content like translations or scores
//! 3. **Formatting** - Colors, styles, and fonts
//! 4. **Interactivity** - Click and hover events
//!
//! See [Minecraft Wiki](https://minecraft.wiki/w/Text_component_format) for full specification.
#![warn(missing_docs)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used, clippy::expect_used)]
#![forbid(missing_copy_implementations, missing_debug_implementations)]
#![forbid(unsafe_code)]

pub mod minimessage;
pub mod parsing;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::{collections::HashMap, fmt, str::FromStr};

/// Represents a Minecraft text component. Allows de/serialization using Serde with JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component {
    /// Simple string component (shorthand for `{text: "value"}`)
    String(String),
    /// Array of components (shorthand for first component with extras)
    Array(Vec<Component>),
    /// Full component object with properties
    Object(Box<ComponentObject>),
}

/// Content type of a component object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// Plain text content
    Text,
    /// Localized translation text
    Translatable,
    /// Scoreboard value
    Score,
    /// Entity selector
    Selector,
    /// Key binding display
    Keybind,
    /// NBT data display
    Nbt,
}

/// Named text colors from Minecraft
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    /// #000000
    Black,
    /// #0000AA
    DarkBlue,
    /// #00AA00
    DarkGreen,
    /// #00AAAA
    DarkAqua,
    /// #AA0000
    DarkRed,
    /// #AA00AA
    DarkPurple,
    /// #FFAA00
    Gold,
    /// #AAAAAA
    Gray,
    /// #555555
    DarkGray,
    /// #5555FF
    Blue,
    /// #55FF55
    Green,
    /// #55FFFF
    Aqua,
    /// #FF5555
    Red,
    /// #FF55FF
    LightPurple,
    /// #FFFF55
    Yellow,
    /// #FFFFFF
    White,
}

impl FromStr for NamedColor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(NamedColor::Black),
            "dark_blue" => Ok(NamedColor::DarkBlue),
            "dark_green" => Ok(NamedColor::DarkGreen),
            "dark_aqua" => Ok(NamedColor::DarkAqua),
            "dark_red" => Ok(NamedColor::DarkRed),
            "dark_purple" => Ok(NamedColor::DarkPurple),
            "gold" => Ok(NamedColor::Gold),
            "gray" => Ok(NamedColor::Gray),
            "dark_gray" => Ok(NamedColor::DarkGray),
            "blue" => Ok(NamedColor::Blue),
            "green" => Ok(NamedColor::Green),
            "aqua" => Ok(NamedColor::Aqua),
            "red" => Ok(NamedColor::Red),
            "light_purple" => Ok(NamedColor::LightPurple),
            "yellow" => Ok(NamedColor::Yellow),
            "white" => Ok(NamedColor::White),
            _ => Err(()),
        }
    }
}

/// Text color representation (either named or hex)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    /// Predefined Minecraft color name
    Named(NamedColor),
    /// Hex color code in #RRGGBB format
    Hex(String),
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Color::Named(named) => named.serialize(serializer),
            Color::Hex(hex) => hex.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Ok(named) = serde_json::from_str::<NamedColor>(&format!("\"{s}\"")) {
            Ok(Color::Named(named))
        } else {
            Ok(Color::Hex(s))
        }
    }
}

/// Shadow color representation (integer or float array)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShadowColor {
    /// RGBA packed as 32-bit integer (0xRRGGBBAA)
    Int(i32),
    /// RGBA as [0.0-1.0] float values
    Floats([f32; 4]),
}

/// Actions triggered when clicking text
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
#[allow(missing_docs)]
pub enum ClickEvent {
    /// Open URL in browser
    OpenUrl { url: String },
    /// Open file (client-side only)
    OpenFile { path: String },
    /// Execute command
    RunCommand { command: String },
    /// Suggest command in chat
    SuggestCommand { command: String },
    /// Change page in books
    ChangePage { page: i32 },
    /// Copy text to clipboard
    CopyToClipboard { value: String },
}

/// UUID representation for entity hover events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UuidRepr {
    /// String representation (hyphenated hex format)
    String(String),
    /// Integer array representation
    IntArray([i32; 4]),
}

/// Information shown when hovering over text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum HoverEvent {
    /// Show text component
    ShowText {
        /// Text to display
        value: Component,
    },
    /// Show item tooltip
    ShowItem {
        /// Item ID (e.g., "minecraft:diamond_sword")
        id: String,
        /// Stack count
        #[serde(skip_serializing_if = "Option::is_none")]
        count: Option<i32>,
        /// Additional item components
        #[serde(skip_serializing_if = "Option::is_none")]
        components: Option<Value>,
    },
    /// Show entity information
    ShowEntity {
        /// Custom name override
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<Component>,
        /// Entity type ID
        id: String,
        /// Entity UUID
        uuid: UuidRepr,
    },
}

/// Scoreboard value content
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreContent {
    /// Score holder (player name or selector)
    pub name: String,
    /// Objective name
    pub objective: String,
}

/// Source for NBT data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NbtSource {
    /// Block entity data
    Block,
    /// Entity data
    Entity,
    /// Command storage
    Storage,
}

/// Core component structure containing all properties
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentObject {
    /// Content type specification
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,

    /// Plain text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Translation key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<String>,

    /// Fallback text for missing translations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,

    /// Arguments for translations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Vec<Component>>,

    /// Scoreboard value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreContent>,

    /// Entity selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,

    /// Custom separator for multi-value components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<Component>>,

    /// Key binding name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybind: Option<String>,

    /// NBT path query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,

    /// NBT source type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<NbtSource>,

    /// Whether to interpret NBT as components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpret: Option<bool>,

    /// Block coordinates for NBT source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,

    /// Entity selector for NBT source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Storage ID for NBT source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    /// Child components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Component>>,

    /// Text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    /// Font resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    /// Bold formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    /// Italic formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    /// Underline formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,

    /// Strikethrough formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    /// Obfuscated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    /// Text shadow color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<ShadowColor>,

    /// Text insertion on shift-click
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,

    /// Click action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,

    /// Hover action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

/// Style properties for components
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    /// Text color
    pub color: Option<Color>,
    /// Font resource location
    pub font: Option<String>,
    /// Bold formatting
    pub bold: Option<bool>,
    /// Italic formatting
    pub italic: Option<bool>,
    /// Underline formatting
    pub underlined: Option<bool>,
    /// Strikethrough formatting
    pub strikethrough: Option<bool>,
    /// Obfuscated text
    pub obfuscated: Option<bool>,
    /// Text shadow color
    pub shadow_color: Option<ShadowColor>,
    /// Text insertion on shift-click
    pub insertion: Option<String>,
    /// Click action
    pub click_event: Option<ClickEvent>,
    /// Hover action
    pub hover_event: Option<HoverEvent>,
}

/// Text decoration styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    /// Bold text
    Bold,
    /// Italic text
    Italic,
    /// Underlined text
    Underlined,
    /// Strikethrough text
    Strikethrough,
    /// Obfuscated (scrambled) text
    Obfuscated,
}

/// Style properties for merging (unused in current implementation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleMerge {
    /// Color property
    Color,
    /// Font property
    Font,
    /// Bold property
    Bold,
    /// Italic property
    Italic,
    /// Underline property
    Underlined,
    /// Strikethrough property
    Strikethrough,
    /// Obfuscated property
    Obfuscated,
    /// Shadow color property
    ShadowColor,
    /// Insertion property
    Insertion,
    /// Click event property
    ClickEvent,
    /// Hover event property
    HoverEvent,
}

impl Component {
    /// Creates a plain text component
    pub fn text(text: impl AsRef<str>) -> Self {
        Component::Object(Box::new(ComponentObject {
            text: Some(text.as_ref().to_string()),
            ..Default::default()
        }))
    }

    /// Appends a child component
    pub fn append<C: Into<Component>>(self, component: C) -> Self {
        let component = component.into();
        match self {
            Component::String(s) => Component::Object(Box::new(ComponentObject {
                content_type: Some(ContentType::Text),
                text: Some(s),
                extra: Some(vec![component]),
                ..Default::default()
            })),
            Component::Array(mut vec) => {
                vec.push(component);
                Component::Array(vec)
            }
            Component::Object(mut obj) => {
                if let Some(extras) = &mut obj.extra {
                    extras.push(component);
                } else {
                    obj.extra = Some(vec![component]);
                }
                Component::Object(obj)
            }
        }
    }

    /// Appends a newline character
    pub fn append_newline(self) -> Self {
        self.append(Component::text("\n"))
    }

    /// Appends a space character
    pub fn append_space(self) -> Self {
        self.append(Component::text(" "))
    }

    /// Gets plain text from component, if it is a string
    pub fn get_plain_text(&self) -> Option<Cow<'_, str>> {
        match self {
            Component::String(s) => Some(Cow::Borrowed(s)),
            Component::Object(obj) => obj.text.as_deref().map(Cow::Borrowed),
            _ => None,
        }
    }

    /// Applies fallback styles to unset properties
    pub fn apply_fallback_style(self, fallback: &Style) -> Self {
        match self {
            Component::String(s) => {
                let mut obj = ComponentObject {
                    content_type: Some(ContentType::Text),
                    text: Some(s),
                    ..Default::default()
                };
                obj.merge_style(fallback);
                Component::Object(Box::new(obj))
            }
            Component::Array(vec) => Component::Array(
                vec.into_iter()
                    .map(|c| c.apply_fallback_style(fallback))
                    .collect(),
            ),
            Component::Object(mut obj) => {
                obj.merge_style(fallback);
                if let Some(extras) = obj.extra {
                    obj.extra = Some(
                        extras
                            .into_iter()
                            .map(|c| c.apply_fallback_style(fallback))
                            .collect(),
                    );
                }
                Component::Object(obj)
            }
        }
    }

    /// Sets text color
    pub fn color(self, color: Option<Color>) -> Self {
        self.map_object(|mut obj| {
            obj.color = color;
            obj
        })
    }

    /// Sets font
    pub fn font(self, font: Option<String>) -> Self {
        self.map_object(|mut obj| {
            obj.font = font;
            obj
        })
    }

    /// Sets text decoration state
    pub fn decoration(self, decoration: TextDecoration, state: Option<bool>) -> Self {
        self.map_object(|mut obj| {
            match decoration {
                TextDecoration::Bold => obj.bold = state,
                TextDecoration::Italic => obj.italic = state,
                TextDecoration::Underlined => obj.underlined = state,
                TextDecoration::Strikethrough => obj.strikethrough = state,
                TextDecoration::Obfuscated => obj.obfuscated = state,
            }
            obj
        })
    }

    /// Sets multiple decorations at once
    pub fn decorations(self, decorations: &HashMap<TextDecoration, Option<bool>>) -> Self {
        self.map_object(|mut obj| {
            for (decoration, state) in decorations {
                match decoration {
                    TextDecoration::Bold => obj.bold = *state,
                    TextDecoration::Italic => obj.italic = *state,
                    TextDecoration::Underlined => obj.underlined = *state,
                    TextDecoration::Strikethrough => obj.strikethrough = *state,
                    TextDecoration::Obfuscated => obj.obfuscated = *state,
                }
            }
            obj
        })
    }

    /// Sets click event
    pub fn click_event(self, event: Option<ClickEvent>) -> Self {
        self.map_object(|mut obj| {
            obj.click_event = event;
            obj
        })
    }

    /// Sets hover event
    pub fn hover_event(self, event: Option<HoverEvent>) -> Self {
        self.map_object(|mut obj| {
            obj.hover_event = event;
            obj
        })
    }

    /// Sets insertion text
    pub fn insertion(self, insertion: Option<String>) -> Self {
        self.map_object(|mut obj| {
            obj.insertion = insertion;
            obj
        })
    }

    /// Checks if a decoration is enabled
    pub fn has_decoration(&self, decoration: TextDecoration) -> bool {
        match self {
            Component::Object(obj) => match decoration {
                TextDecoration::Bold => obj.bold.unwrap_or(false),
                TextDecoration::Italic => obj.italic.unwrap_or(false),
                TextDecoration::Underlined => obj.underlined.unwrap_or(false),
                TextDecoration::Strikethrough => obj.strikethrough.unwrap_or(false),
                TextDecoration::Obfuscated => obj.obfuscated.unwrap_or(false),
            },
            _ => false,
        }
    }

    /// Checks if any styling is present
    pub fn has_styling(&self) -> bool {
        match self {
            Component::Object(obj) => {
                obj.color.is_some()
                    || obj.font.is_some()
                    || obj.bold.is_some()
                    || obj.italic.is_some()
                    || obj.underlined.is_some()
                    || obj.strikethrough.is_some()
                    || obj.obfuscated.is_some()
                    || obj.shadow_color.is_some()
                    || obj.insertion.is_some()
                    || obj.click_event.is_some()
                    || obj.hover_event.is_some()
            }
            _ => false,
        }
    }

    /// Sets child components
    pub fn set_children(self, children: Vec<Component>) -> Self {
        self.map_object(|mut obj| {
            obj.extra = Some(children);
            obj
        })
    }

    /// Gets child components
    pub fn get_children(&self) -> &[Component] {
        match self {
            Component::Object(obj) => obj.extra.as_deref().unwrap_or_default(),
            Component::Array(vec) => vec.as_slice(),
            Component::String(_) => &[],
        }
    }

    /// Internal method to apply transformations to component objects
    fn map_object<F>(self, f: F) -> Self
    where
        F: FnOnce(ComponentObject) -> ComponentObject,
    {
        match self {
            Component::String(s) => {
                let obj = ComponentObject {
                    content_type: Some(ContentType::Text),
                    text: Some(s),
                    ..Default::default()
                };
                Component::Object(Box::new(f(obj)))
            }
            Component::Array(vec) => {
                let mut obj = ComponentObject {
                    extra: Some(vec),
                    ..Default::default()
                };
                obj = f(obj);
                Component::Object(Box::new(obj))
            }
            Component::Object(obj) => Component::Object(Box::new(f(*obj))),
        }
    }
}

impl ComponentObject {
    /// Merges style properties from a fallback style
    fn merge_style(&mut self, fallback: &Style) {
        if self.color.is_none() {
            self.color = fallback.color.clone();
        }
        if self.font.is_none() {
            self.font = fallback.font.clone();
        }
        if self.bold.is_none() {
            self.bold = fallback.bold;
        }
        if self.italic.is_none() {
            self.italic = fallback.italic;
        }
        if self.underlined.is_none() {
            self.underlined = fallback.underlined;
        }
        if self.strikethrough.is_none() {
            self.strikethrough = fallback.strikethrough;
        }
        if self.obfuscated.is_none() {
            self.obfuscated = fallback.obfuscated;
        }
        if self.shadow_color.is_none() {
            self.shadow_color = fallback.shadow_color;
        }
        if self.insertion.is_none() {
            self.insertion = fallback.insertion.clone();
        }
        if self.click_event.is_none() {
            self.click_event = fallback.click_event.clone();
        }
        if self.hover_event.is_none() {
            self.hover_event = fallback.hover_event.clone();
        }
    }
}

/// Error type for color parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseColorError;

impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid color format")
    }
}

impl std::error::Error for ParseColorError {}

fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    let s = s.strip_prefix('#')?;
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        return Some([r, g, b]);
    }
    None
}

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if parse_hex_color(s).is_none() {
            return Err(ParseColorError);
        }
        Ok(Color::Hex(s.to_string()))
    }
}

impl<T: AsRef<str>> From<T> for Component {
    fn from(value: T) -> Component {
        let s: &str = value.as_ref();
        Component::String(s.to_string())
    }
}

impl fmt::Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NamedColor::Black => "black",
            NamedColor::DarkBlue => "dark_blue",
            NamedColor::DarkGreen => "dark_green",
            NamedColor::DarkAqua => "dark_aqua",
            NamedColor::DarkRed => "dark_red",
            NamedColor::DarkPurple => "dark_purple",
            NamedColor::Gold => "gold",
            NamedColor::Gray => "gray",
            NamedColor::DarkGray => "dark_gray",
            NamedColor::Blue => "blue",
            NamedColor::Green => "green",
            NamedColor::Aqua => "aqua",
            NamedColor::Red => "red",
            NamedColor::LightPurple => "light_purple",
            NamedColor::Yellow => "yellow",
            NamedColor::White => "white",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let raw_json = r#"
        {
          "text": "Hello, ",
          "color": "yellow",
          "extra": [
            {
              "text": "World!",
              "color": "white",
              "bold": true
            },
            {
              "translate": "chat.type.say",
              "with": [
                { "selector": "@p" }
              ]
            }
          ]
        }
        "#;

        let component: Component = serde_json::from_str(raw_json).unwrap();
        println!("Message: {component:#?}");
    }
}
