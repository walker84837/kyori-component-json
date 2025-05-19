use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Component {
    String(String),
    Array(Vec<Component>),
    Object(Box<ComponentObject>),
}

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
pub enum UuidRepr {
    String(String),
    IntArray([i32; 4]),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "action")]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
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

impl Component {
    pub fn text(text: impl AsRef<str>) -> Self {
        Component::Object(Box::new(ComponentObject {
            content_type: Some(ContentType::Text),
            text: Some(text.as_ref().to_string()),
            ..Default::default()
        }))
    }
}

impl Default for ComponentObject {
    fn default() -> Self {
        Self {
            content_type: None,
            text: None,
            translate: None,
            fallback: None,
            with: None,
            score: None,
            selector: None,
            separator: None,
            keybind: None,
            nbt: None,
            source: None,
            interpret: None,
            block: None,
            entity: None,
            storage: None,
            extra: None,
            color: None,
            font: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            shadow_color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
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
