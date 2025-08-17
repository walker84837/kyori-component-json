//! MiniMessage format parser and serializer for Minecraft components.
//!
//! Implements the [`ComponentParser`] and [`ComponentSerializer`] traits
//! for the MiniMessage text format.

use crate::parsing::{ComponentParser, ComponentSerializer};
use crate::{
    ClickEvent, Color, Component, ComponentObject, HoverEvent, NamedColor, Style, TextDecoration,
};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during MiniMessage parsing/serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiniMessageError(String);

impl fmt::Display for MiniMessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MiniMessage error: {}", self.0)
    }
}

impl Error for MiniMessageError {}

/// Configuration for MiniMessage parsing/serialization.
#[derive(Debug, Clone, Default, PartialEq, Eq, Copy, Hash)]
pub struct MiniMessageConfig {
    /// Whether to use strict parsing (requires proper tag closing)
    pub strict: bool,
    /// Whether to parse legacy color codes (e.g., &6 for gold)
    pub parse_legacy_colors: bool,
}

/// MiniMessage parser and serializer implementation.
#[derive(Debug, Clone, Copy)]
pub struct MiniMessage {
    config: MiniMessageConfig,
}

impl MiniMessage {
    /// Creates a new MiniMessage instance with default configuration.
    pub fn new() -> Self {
        Self::with_config(Default::default())
    }

    /// Creates a new MiniMessage instance with custom configuration.
    pub fn with_config(config: MiniMessageConfig) -> Self {
        MiniMessage { config }
    }

    /// Parse input using instance configuration
    pub fn parse(&self, input: impl AsRef<str>) -> Result<Component, MiniMessageError> {
        let mut parser = Parser::new(input.as_ref(), &self.config);
        parser.parse()
    }
}

impl Default for MiniMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentParser for MiniMessage {
    type Err = MiniMessageError;

    /// Parse input from MiniMessage string to Component using default configuration.
    fn from_string(input: impl AsRef<str>) -> Result<Component, Self::Err> {
        let mm = MiniMessage::new();
        mm.parse(input)
    }
}

impl ComponentSerializer for MiniMessage {
    type Err = MiniMessageError;

    fn to_string(component: &Component) -> Result<String, Self::Err> {
        Serializer::new().serialize(component)
    }
}

/// Internal parser state
struct Parser<'a> {
    input: &'a str,
    position: usize,
    config: &'a MiniMessageConfig,
    style_stack: Vec<Style>,
    component_parts: Vec<Component>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str, config: &'a MiniMessageConfig) -> Self {
        Self {
            input,
            position: 0,
            config,
            style_stack: vec![Style::default()],
            component_parts: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<Component, MiniMessageError> {
        while self.position < self.input.len() {
            if self.starts_with('<') {
                self.parse_tag()?;
            } else {
                self.parse_text()?;
            }
        }

        let parts = std::mem::take(&mut self.component_parts);
        if parts.len() == 1 {
            // SAFETY: This is safe because we always have at least one style
            Ok(parts.into_iter().next().unwrap())
        } else {
            Ok(Component::Array(parts))
        }
    }

    fn parse_text(&mut self) -> Result<(), MiniMessageError> {
        let start = self.position;
        while self.position < self.input.len() {
            if self.starts_with('<') || (self.config.parse_legacy_colors && self.starts_with('&')) {
                break;
            }
            self.position += 1;
        }

        if start < self.position {
            let text = &self.input[start..self.position];
            let current_style = self.current_style();
            let mut comp = Component::text(text);
            comp = comp.color(current_style.color.clone());
            comp = comp.decorations(&self.collect_decorations());
            self.component_parts.push(comp);
        }
        Ok(())
    }

    fn parse_tag(&mut self) -> Result<(), MiniMessageError> {
        // Skip '<'
        self.position += 1;

        if self.starts_with('/') {
            // Closing tag
            self.position += 1;
            let tag_name = self.read_tag_name()?;
            self.handle_close_tag(&tag_name)?;
            self.expect('>')?;
        } else {
            // Opening tag
            let tag_name = self.read_tag_name()?;
            let mut args = Vec::new();
            let mut self_closing = false;

            while self.position < self.input.len() {
                // skip whitespace
                self.skip_whitespace();
                // skip colon separators
                while self.starts_with(':') {
                    self.position += 1;
                    self.skip_whitespace();
                }

                // if we’ve hit the end of the tag, stop
                if self.starts_with('>') || self.starts_with('/') {
                    break;
                }

                // read an argument
                let arg = self.read_argument()?;
                args.push(arg);
            }

            // Check for self-closing tag
            if self.starts_with('/') {
                self.position += 1;
                self_closing = true;
            }
            self.expect('>')?;

            self.handle_open_tag(&tag_name, args, self_closing)?;
        }

        Ok(())
    }

    fn read_tag_name(&mut self) -> Result<String, MiniMessageError> {
        let start = self.position;
        while self.position < self.input.len() {
            let c = self.current_char();
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
                break;
            }
            self.position += 1;
        }
        if start == self.position {
            return Err(MiniMessageError("Expected tag name".to_string()));
        }
        Ok(self.input[start..self.position].to_lowercase())
    }

    fn read_argument(&mut self) -> Result<String, MiniMessageError> {
        if self.starts_with('\'') || self.starts_with('"') {
            self.read_quoted_string()
        } else {
            self.read_unquoted_string()
        }
    }

    fn read_quoted_string(&mut self) -> Result<String, MiniMessageError> {
        let quote_char = self.current_char();
        self.position += 1;

        let mut escaped = false;
        let mut result = String::new();

        while self.position < self.input.len() {
            let c = self.current_char();
            if escaped {
                result.push(c);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == quote_char {
                self.position += 1;
                return Ok(result);
            } else {
                result.push(c);
            }
            self.position += 1;
        }

        Err(MiniMessageError("Unterminated quoted string".to_string()))
    }

    fn read_unquoted_string(&mut self) -> Result<String, MiniMessageError> {
        let start = self.position;
        while self.position < self.input.len() {
            let c = self.current_char();
            if c == ':' || c == '>' || c == '/' || c.is_whitespace() {
                break;
            }
            self.position += 1;
        }
        // what?
        // if start == self.position {
        //     return Err(MiniMessageError("Expected argument".to_string()));
        // }
        Ok(self.input[start..self.position].to_string())
    }

    fn handle_open_tag(
        &mut self,
        tag: &str,
        args: Vec<String>,
        self_closing: bool,
    ) -> Result<(), MiniMessageError> {
        match tag {
            // Colors
            "black" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Black)))?,
            "dark_blue" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkBlue)))?
            }
            "dark_green" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkGreen)))?
            }
            "dark_aqua" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkAqua)))?
            }
            "dark_red" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkRed)))?,
            "dark_purple" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkPurple)))?
            }
            "gold" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Gold)))?,
            "gray" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Gray)))?,
            "dark_gray" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::DarkGray)))?
            }
            "blue" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Blue)))?,
            "green" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Green)))?,
            "aqua" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Aqua)))?,
            "red" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Red)))?,
            "light_purple" => {
                self.push_style(|s| s.color = Some(Color::Named(NamedColor::LightPurple)))?
            }
            "yellow" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::Yellow)))?,
            "white" => self.push_style(|s| s.color = Some(Color::Named(NamedColor::White)))?,
            "color" | "colour" | "c" if !args.is_empty() => {
                if let Ok(color) = args[0].parse::<Color>() {
                    self.push_style(|s| s.color = Some(color))?
                }
            }

            // Decorations
            "bold" | "b" => self.push_style(|s| s.bold = Some(true))?,
            "italic" | "i" | "em" => self.push_style(|s| s.italic = Some(true))?,
            "underlined" | "u" => self.push_style(|s| s.underlined = Some(true))?,
            "strikethrough" | "st" => self.push_style(|s| s.strikethrough = Some(true))?,
            "obfuscated" | "obf" => self.push_style(|s| s.obfuscated = Some(true))?,

            // Reset tag
            "reset" => self.reset_style()?,

            // Click events
            "click" if args.len() >= 2 => {
                let action = args[0].as_str();
                let value = args[1].as_str();
                match action {
                    "run_command" => self.push_style(|s| {
                        s.click_event = Some(ClickEvent::RunCommand {
                            command: value.to_string(),
                        })
                    })?,
                    "suggest_command" => self.push_style(|s| {
                        s.click_event = Some(ClickEvent::SuggestCommand {
                            command: value.to_string(),
                        })
                    })?,
                    "open_url" => self.push_style(|s| {
                        s.click_event = Some(ClickEvent::OpenUrl {
                            url: value.to_string(),
                        })
                    })?,
                    "copy_to_clipboard" => self.push_style(|s| {
                        s.click_event = Some(ClickEvent::CopyToClipboard {
                            value: value.to_string(),
                        })
                    })?,
                    _ => {}
                }
            }

            // Hover events
            "hover" if !args.is_empty() => {
                let action = args[0].as_str();
                if action == "show_text" && args.len() >= 2 {
                    let mut nested_parser = Parser::new(&args[1], self.config);
                    let nested = nested_parser.parse()?;
                    self.push_style(|s| {
                        s.hover_event = Some(HoverEvent::ShowText { value: nested })
                    })?;
                }
            }

            // Newline
            "newline" | "br" => {
                self.component_parts.push(Component::text("\n"));
            }

            // Insertion
            "insert" | "insertion" if !args.is_empty() => {
                self.push_style(|s| s.insertion = Some(args[0].clone()))?
            }

            // Handle self-closing tags
            _ if self_closing => {
                // For self-closing tags, create an empty component with the style
                let current_style = self.current_style();
                let mut comp = Component::text("");
                comp = comp.color(current_style.color.clone());
                comp = comp.decorations(&self.collect_decorations());
                self.component_parts.push(comp);
            }

            // Unknown tags are treated as text
            _ => {
                let mut tag_text = format!("<{tag}");
                for arg in args {
                    tag_text.push(':');
                    tag_text.push_str(&arg);
                }
                if self_closing {
                    tag_text.push('/');
                }
                tag_text.push('>');
                self.component_parts
                    .push(Component::text(tag_text).apply_fallback_style(self.current_style()));
            }
        }

        Ok(())
    }

    fn handle_close_tag(&mut self, tag: &str) -> Result<(), MiniMessageError> {
        match tag {
            "bold" | "b" | "italic" | "i" | "em" | "underlined" | "u" | "strikethrough" | "st"
            | "obfuscated" | "obf" | "color" | "colour" | "c" | "click" | "hover" | "insert"
            | "insertion" => {
                self.pop_style()?;
            }
            _ => {
                // For unknown tags, just pop the style anyway
                if self.style_stack.len() > 1 {
                    self.style_stack.pop();
                }
            }
        }
        Ok(())
    }

    fn push_style<F>(&mut self, modifier: F) -> Result<(), MiniMessageError>
    where
        F: FnOnce(&mut Style),
    {
        let mut new_style = self.current_style().clone();
        modifier(&mut new_style);
        self.style_stack.push(new_style);
        Ok(())
    }

    fn pop_style(&mut self) -> Result<(), MiniMessageError> {
        if self.style_stack.len() > 1 {
            self.style_stack.pop();
            Ok(())
        } else {
            Err(MiniMessageError("Unbalanced closing tag".to_string()))
        }
    }

    fn reset_style(&mut self) -> Result<(), MiniMessageError> {
        while self.style_stack.len() > 1 {
            self.style_stack.pop();
        }
        Ok(())
    }

    fn current_style(&self) -> &Style {
        // SAFETY: This is safe because we always have at least one style
        self.style_stack.last().unwrap()
    }

    fn collect_decorations(&self) -> HashMap<TextDecoration, Option<bool>> {
        let style = self.current_style();
        let mut decorations = HashMap::new();
        if let Some(bold) = style.bold {
            decorations.insert(TextDecoration::Bold, Some(bold));
        }
        if let Some(italic) = style.italic {
            decorations.insert(TextDecoration::Italic, Some(italic));
        }
        if let Some(underlined) = style.underlined {
            decorations.insert(TextDecoration::Underlined, Some(underlined));
        }
        if let Some(strikethrough) = style.strikethrough {
            decorations.insert(TextDecoration::Strikethrough, Some(strikethrough));
        }
        if let Some(obfuscated) = style.obfuscated {
            decorations.insert(TextDecoration::Obfuscated, Some(obfuscated));
        }
        decorations
    }

    fn starts_with(&self, c: char) -> bool {
        self.input[self.position..].starts_with(c)
    }

    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            if !self.input[self.position..].starts_with(char::is_whitespace) {
                break;
            }
            self.position += 1;
        }
    }

    fn expect(&mut self, c: char) -> Result<(), MiniMessageError> {
        if self.position < self.input.len() && self.current_char() == c {
            self.position += 1;
            Ok(())
        } else {
            Err(MiniMessageError(format!("Expected '{c}'")))
        }
    }
}

/// Serializes components to MiniMessage format
struct Serializer {
    output: String,
    current_style: Style,
}

impl Serializer {
    fn new() -> Self {
        Self {
            output: String::new(),
            current_style: Style::default(),
        }
    }

    fn serialize(&mut self, component: &Component) -> Result<String, MiniMessageError> {
        self.serialize_component(component)?;
        Ok(self.output.clone())
    }

    fn serialize_component(&mut self, component: &Component) -> Result<(), MiniMessageError> {
        match component {
            Component::String(s) => self.serialize_text(s),
            Component::Array(components) => {
                let base_style = self.current_style.clone();
                for comp in components {
                    // Reset to base style before each component
                    self.current_style = base_style.clone();
                    self.serialize_component(comp)?;
                }
                Ok(())
            }
            Component::Object(obj) => self.serialize_object(obj),
        }
    }

    fn serialize_object(&mut self, obj: &ComponentObject) -> Result<(), MiniMessageError> {
        // Save current style to compare changes
        let prev_style = self.current_style.clone();

        // Apply style changes
        let mut style_changes = Vec::new();

        if let Some(color) = &obj.color
            && Some(color) != prev_style.color.as_ref()
        {
            if let Some(named) = color.to_named() {
                style_changes.push(named.to_string());
            } else if let Color::Hex(hex) = color {
                style_changes.push(format!("color:{hex}"));
            }
        }

        if obj.bold != prev_style.bold && obj.bold == Some(true) {
            style_changes.push("bold".to_string());
        }

        if obj.italic != prev_style.italic && obj.italic == Some(true) {
            style_changes.push("italic".to_string());
        }

        if obj.underlined != prev_style.underlined && obj.underlined == Some(true) {
            style_changes.push("underlined".to_string());
        }

        if obj.strikethrough != prev_style.strikethrough && obj.strikethrough == Some(true) {
            style_changes.push("strikethrough".to_string());
        }

        if obj.obfuscated != prev_style.obfuscated && obj.obfuscated == Some(true) {
            style_changes.push("obfuscated".to_string());
        }

        // Apply style changes
        for change in &style_changes {
            self.output.push_str(&format!("<{change}>"));
        }

        // Update current style
        self.current_style = Style {
            color: obj.color.clone(),
            bold: obj.bold,
            italic: obj.italic,
            underlined: obj.underlined,
            strikethrough: obj.strikethrough,
            obfuscated: obj.obfuscated,
            ..self.current_style.clone()
        };

        // Serialize text content
        if let Some(text) = &obj.text {
            self.serialize_text(text)?;
        }

        // Serialize children
        if let Some(extra) = &obj.extra {
            for comp in extra {
                self.serialize_component(comp)?;
            }
        }

        // Close style changes
        for change in style_changes.iter().rev() {
            self.output.push_str(&format!("</{change}>"));
        }

        // Restore previous style
        self.current_style = prev_style;

        Ok(())
    }

    fn serialize_text(&mut self, text: &str) -> Result<(), MiniMessageError> {
        // Escape special characters
        for c in text.chars() {
            match c {
                '<' => self.output.push_str("&lt;"),
                '>' => self.output.push_str("&gt;"),
                '&' => self.output.push_str("&amp;"),
                _ => self.output.push(c),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Component, NamedColor};

    #[test]
    fn test_parse_simple() {
        let mm = MiniMessage::new();
        let comp = mm.parse("Hello <red>world</red>!").unwrap();

        if let Component::Array(parts) = comp {
            assert_eq!(parts.len(), 3);
            assert_eq!(parts[0].get_plain_text().unwrap(), "Hello ");
            assert_eq!(parts[1].get_plain_text().unwrap(), "world");
            assert_eq!(parts[2].get_plain_text().unwrap(), "!");
        } else {
            panic!("Expected array component");
        }
    }

    #[test]
    fn test_parse_nested() {
        let mm = MiniMessage::new();
        let comp = mm
            .parse("Click <hover:show_text:'<red>Action!'>here</hover>")
            .expect("Failed to parse component");

        // Verify hover event exists and contains a red text component
        if let Component::Object(obj) = &comp
            && let Some(children) = &obj.extra
        {
            if let Component::Object(hover_obj) = &children[1]
                && let Some(hover_event) = &hover_obj.hover_event
            {
                match hover_event {
                    HoverEvent::ShowText { value } => {
                        assert_eq!(value.get_plain_text().unwrap(), "Action!");
                    }
                    _ => panic!("Expected show_text hover event"),
                }
            }
        }
    }

    #[test]
    fn test_serialize_simple() {
        let comp = Component::text("Hello ")
            .color(Some(Color::Named(NamedColor::Yellow)))
            .append(Component::text("world").color(Some(Color::Named(NamedColor::Red))));

        let result = MiniMessage::to_string(&comp).unwrap();
        // TODO: is <yellow>Hello </yellow><red>world</red> technically correct?
        assert_eq!(result, "<yellow>Hello <red>world</red></yellow>");
    }

    // TODO: comprehensive tests would involve traversing the parsed MiniMessage's tree
    #[test]
    fn test_readme_example_basic_red_text() {
        let mm = MiniMessage::new();
        let comp = mm.parse("<red>Hello, world!</red>").unwrap();
        // Assert that the component is a text component with red color
        if let Component::Object(obj) = comp {
            assert_eq!(obj.text, Some("Hello, world!".to_string()));
            assert_eq!(obj.color, Some(Color::Named(NamedColor::Red)));
        } else {
            panic!("Expected object component");
        }
    }

    #[test]
    fn test_readme_example_nested_colors_and_bold() {
        let mm = MiniMessage::new();
        let comp = mm
            .parse("<green>This is <blue>blue</blue> and <b>bold</b>!</green>")
            .unwrap();

        assert_eq!(comp.to_plain_text(), "This is blue and bold!");
    }

    #[test]
    fn test_readme_example_click_and_hover_events() {
        let mm = MiniMessage::new();
        let comp = mm.parse("<hover:show_text:\"<red>Hover Text</red>\"><click:open_url:\"https://example.com\">Clickable Link</click></hover>").unwrap();
        assert_eq!(comp.to_plain_text(), "Clickable Link");
    }
}
