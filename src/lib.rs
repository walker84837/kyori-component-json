#![warn(missing_docs)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// A Component is an immutable object that represents how text is displayed Minecraft clients.
pub enum Component {
    /// Simple text
    String(String),
    /// A list of Components
    Array(Vec<Component>),
    /// A single Component object
    Object(Box<ComponentObject>),
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Translatable,
    Score,
    Selector,
    Keybind,
    Nbt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
/// The named text colours in Minecraft: Java Edition.
#[allow(missing_docs)]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

#[derive(Debug, Clone)]
pub enum Color {
    Named(NamedColor),
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
        if let Ok(named) = serde_json::from_str::<NamedColor>(&format!("\"{}\"", s)) {
            Ok(Color::Named(named))
        } else {
            Ok(Color::Hex(s))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ShadowColor {
    Int(i32),
    Floats([f32; 4]),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "action")]
/// What happens when you click on text which has a click event
pub enum ClickEvent {
    OpenUrl { url: String },
    OpenFile { path: String },
    RunCommand { command: String },
    SuggestCommand { command: String },
    ChangePage { page: i32 },
    CopyToClipboard { value: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Possible UUID representations from a Component
pub enum UuidRepr {
    String(String),
    IntArray([i32; 4]),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "action")]
/// What is shown when hovering text that has a HoverEvent
pub enum HoverEvent {
    ShowText {
        value: Component,
    },
    ShowItem {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        count: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        components: Option<Value>,
    },
    ShowEntity {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<Component>,
        id: String,
        uuid: UuidRepr,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreContent {
    pub name: String,
    pub objective: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NbtSource {
    Block,
    Entity,
    Storage,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// Actual contents of a [`Component`]
pub struct ComponentObject {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<NbtSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpret: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<ShadowColor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub color: Option<Color>,
    pub font: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub shadow_color: Option<ShadowColor>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleMerge {
    Color,
    Font,
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
    ShadowColor,
    Insertion,
    ClickEvent,
    HoverEvent,
}

impl Component {
    pub fn text(text: impl AsRef<str>) -> Self {
        Component::Object(Box::new(ComponentObject {
            content_type: Some(ContentType::Text),
            text: Some(text.as_ref().to_string()),
            ..Default::default()
        }))
    }

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

    pub fn append_newline(self) -> Self {
        self.append(Component::text("\n"))
    }

    pub fn append_space(self) -> Self {
        self.append(Component::text(" "))
    }

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

    pub fn color(self, color: Option<Color>) -> Self {
        self.map_object(|mut obj| {
            obj.color = color;
            obj
        })
    }

    pub fn font(self, font: Option<String>) -> Self {
        self.map_object(|mut obj| {
            obj.font = font;
            obj
        })
    }

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

    pub fn click_event(self, event: Option<ClickEvent>) -> Self {
        self.map_object(|mut obj| {
            obj.click_event = event;
            obj
        })
    }

    pub fn hover_event(self, event: Option<HoverEvent>) -> Self {
        self.map_object(|mut obj| {
            obj.hover_event = event;
            obj
        })
    }

    pub fn insertion(self, insertion: Option<String>) -> Self {
        self.map_object(|mut obj| {
            obj.insertion = insertion;
            obj
        })
    }

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

    pub fn set_children(self, children: Vec<Component>) -> Self {
        self.map_object(|mut obj| {
            obj.extra = Some(children);
            obj
        })
    }

    pub fn get_children(&self) -> &[Component] {
        match self {
            Component::Object(obj) => obj.extra.as_deref().unwrap_or_default(),
            Component::Array(vec) => vec.as_slice(),
            Component::String(_) => &[],
        }
    }

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
            self.shadow_color = fallback.shadow_color.clone();
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

#[derive(Debug, Clone)]
/// An error with parsing an hex color
pub struct ParseColorError;

impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid color")
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
        if let None = parse_hex_color(s) {
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
        write!(f, "{}", s)
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
