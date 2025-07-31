//! Provides traits for parsing and serializing Minecraft text [`Component`]s.
//!
//! This module defines two core traits, [`ComponentParser`] and [`ComponentSerializer`],
//! which abstract the conversion between string representations and the internal
//! `Component` data structure.
//!
//! # Overview
//!
//! - [`ComponentParser`] allows parsing a string into a [`Component`].
//! - [`ComponentSerializer`] allows serializing a [`Component`] back into a string.
//!
//! These traits are intended to empower developers to implement conversions from and to other formats
//! without tying your code to a specific serialization or parsing library.
//!
//! # Use cases
//!
//! - Implement `ComponentParser` to convert from a user-provided formatted string (e.g., Discord Markdown)
//!   into a `Component`.
//! - Implement `ComponentSerializer` to convert your `Component` back into a string format
//!   (like a binary representation or custom formats) for storage, transmission, or display.
//!
use crate::Component;

/// A trait for parsing a string into a [`Component`].
pub trait ComponentParser {
    /// Error type returned when parsing fails.
    type Err;

    /// Parses a string input into a [`Component`].
    ///
    /// # Parameters
    ///
    /// - `input`: A string slice or any type that can be referenced as a string.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a parsed [`Component`] on success,
    /// or an error of type [`Self::Err`] on failure.
    fn from_string(input: impl AsRef<str>) -> Result<Component, Self::Err>;
}

/// A trait for serializing a [`Component`] into a string representation.
pub trait ComponentSerializer {
    /// Error type returned when serialization fails.
    type Err;

    /// Serializes a [`Component`] into a string representation.
    ///
    /// # Parameters
    ///
    /// - `component`: A reference to the [`Component`] to serialize.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the serialized string on success,
    /// or an error of type [`Self::Err`] on failure.
    fn to_string(component: &Component) -> Result<String, Self::Err>;
}
