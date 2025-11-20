//! Macros for creating components.
//!
//! This module provides the `component!` macro, which offers a declarative way to construct
//! Kyori `Component` objects. It simplifies the process of creating complex components
//! by allowing properties to be specified in a more readable, attribute-like syntax.

#[doc(hidden)]
#[macro_export]
/// Helper macro to convert snake_case identifiers to the corresponding `ClickEvent` enum variants.
/// This allows for a more natural syntax within the `component!` macro for defining click events.
macro_rules! __click_event_from_snake {
    (open_url, { $($body:tt)* }) => { $crate::ClickEvent::OpenUrl { $($body)* } };
    (open_file, { $($body:tt)* }) => { $crate::ClickEvent::OpenFile { $($body)* } };
    (run_command, { $($body:tt)* }) => { $crate::ClickEvent::RunCommand { $($body)* } };
    (suggest_command, { $($body:tt)* }) => { $crate::ClickEvent::SuggestCommand { $($body)* } };
    (change_page, { $($body:tt)* }) => { $crate::ClickEvent::ChangePage { $($body)* } };
    (copy_to_clipboard, { $($body:tt)* }) => { $crate::ClickEvent::CopyToClipboard { $($body)* } };
}

#[doc(hidden)]
#[macro_export]
/// Helper macro to convert snake_case identifiers to the corresponding `HoverEvent` enum variants.
/// Similar to `__click_event_from_snake!`, this simplifies the syntax for defining hover events.
macro_rules! __hover_event_from_snake {
    (show_text, { $($body:tt)* }) => { $crate::HoverEvent::ShowText { value: $($body)* } };
    (show_item, { $($body:tt)* }) => { $crate::HoverEvent::ShowItem { $($body)* } };
    (show_entity, { $($body:tt)* }) => { $crate::HoverEvent::ShowEntity { $($body)* } };
}

#[macro_export]
/// Creates a component in a declarative way.
///
/// The `component!` macro supports two primary forms:
/// 1. `component!(text: "Hello")`: Creates a simple text component.
/// 2. `component!(text: "Hello", { ... properties ... })`: Creates a text component
///    and then applies various properties to it, such as color, decorations, events,
///    and appended components.
///
/// The macro uses an internal "muncher" pattern (`@munch` rules) to iteratively process
/// the provided properties. This allows for a flexible order of properties.
///
/// # Examples
///
/// ```
/// use kyori_component_json::{component, ClickEvent, Color, Component, NamedColor, TextDecoration};
///
/// let component = component!(text: "Hello, ", {
///     color: yellow,
///     append: (component!(text: "World!", {
///         color: white,
///         decoration: bold & true,
///     })),
///     font: "uniform",
///     insertion: "inserted text",
///     click_event: run_command { command: "/say hi".to_string() },
/// });
///
/// let component2 = component!(text: "hello world", {
///    color: #037429,
/// });
/// ```
macro_rules! component {
    // Base case: Creates a simple text component without additional properties.
    (text: $text:expr) => {
        $crate::Component::text($text)
    };

    // Entry point for components with properties:
    // Initializes a base text component and then delegates to the internal `@munch` rules
    // to process the remaining properties iteratively.
    (text: $text:expr, { $($body:tt)* }) => {
        {
            let component = $crate::Component::text($text);
            // Start the muncher with the initial component and all properties.
            component!(@munch component, $($body)*)
        }
    };

    // --- Muncher Rules (@munch) ---
    // The muncher pattern works by repeatedly matching and consuming one property
    // at a time, modifying the `comp` (Component) variable, and then recursively
    // calling itself with the remaining properties (`$rest`).

    // Base case for the muncher: When no more properties are left, return the
    // accumulated component.
    (@munch $comp:ident, ) => { $comp };

    // Rule for named colors (e.g., `color: yellow`):
    // Parses the color identifier, applies it to the component, and continues munching.
    (@munch $comp:ident, color: $color:ident, $($rest:tt)*) => {
        {
            let comp = $comp.color(Some($crate::Color::Named(stringify!($color).parse().unwrap())));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for named colors when it's the last property.
    (@munch $comp:ident, color: $color:ident) => {
        $comp.color(Some($crate::Color::Named(stringify!($color).parse().unwrap())))
    };

    // Rule for hex colors (e.g., `color: #FFFFFF`):
    // Parses the hex code, applies it, and continues munching.
    (@munch $comp:ident, color: #$hex:literal, $($rest:tt)*) => {
        {
            let comp = $comp.color(Some(stringify!(#$hex).replace(" ", "").parse().unwrap()));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for hex colors when it's the last property.
    (@munch $comp:ident, color: #$hex:literal) => {
        $comp.color(Some(stringify!(#$hex).replace(" ", "").parse().unwrap()))
    };

    // Rule for text decorations (e.g., `decoration: bold & true`):
    // Parses the decoration and its state, applies it, and continues munching.
    (@munch $comp:ident, decoration: $deco:ident & $state:expr, $($rest:tt)*) => {
        {
            let deco = stringify!($deco).parse().unwrap();
            let comp = $comp.decoration(deco, Some($state));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for decorations when it's the last property.
    (@munch $comp:ident, decoration: $deco:ident & $state:expr) => {
        {
            let deco = stringify!($deco).parse().unwrap();
            $comp.decoration(deco, Some($state))
        }
    };

    // Rule for font property (e.g., `font: "uniform"`):
    // Applies the font string and continues munching.
    (@munch $comp:ident, font: $value:literal, $($rest:tt)*) => {
        {
            let comp = $comp.font(Some($value.to_string()));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for font when it's the last property.
    (@munch $comp:ident, font: $value:literal) => {
        $comp.font(Some($value.to_string()))
    };

    // Rule for insertion property (e.g., `insertion: "text"`):
    // Applies the insertion string and continues munching.
    (@munch $comp:ident, insertion: $value:literal, $($rest:tt)*) => {
        {
            let comp = $comp.insertion(Some($value.to_string()));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for insertion when it's the last property.
    (@munch $comp:ident, insertion: $value:literal) => {
        $comp.insertion(Some($value.to_string()))
    };

    // Rule for click events (e.g., `click_event: run_command { command: "..." }`):
    // Uses the `__click_event_from_snake!` helper to construct the `ClickEvent`,
    // applies it, and continues munching.
    (@munch $comp:ident, click_event: $type:ident { $($body:tt)* }, $($rest:tt)*) => {
        {
            let event = $crate::__click_event_from_snake!($type, { $($body)* });
            let comp = $comp.click_event(Some(event));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for click events when it's the last property.
    (@munch $comp:ident, click_event: $type:ident { $($body:tt)* }) => {
        {
            let event = $crate::__click_event_from_snake!($type, { $($body)* });
            $comp.click_event(Some(event))
        }
    };

    // Rule for hover events (e.g., `hover_event: show_text { component!(...) }`):
    // Uses the `__hover_event_from_snake!` helper to construct the `HoverEvent`,
    // applies it, and continues munching.
    (@munch $comp:ident, hover_event: $type:ident { $($body:tt)* }, $($rest:tt)*) => {
        {
            let event = $crate::__hover_event_from_snake!($type, { $($body)* });
            let comp = $comp.hover_event(Some(event));
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for hover events when it's the last property.
    (@munch $comp:ident, hover_event: $type:ident { $($body:tt)* }) => {
        {
            let event = $crate::__hover_event_from_snake!($type, { $($body)* });
            $comp.hover_event(Some(event))
        }
    };

    // Generic rule for other fields (legacy or less common, e.g., `append: (component!(...))`).
    // This allows calling methods directly on the component.
    (@munch $comp:ident, $field:ident : ($($value:expr),*), $($rest:tt)*) => {
        {
            let comp = $comp.$field($($value),*);
            component!(@munch comp, $($rest)*)
        }
    };
    // Variant for generic fields when it's the last property.
    (@munch $comp:ident, $field:ident : ($($value:expr),*)) => {
        $comp.$field($($value),*)
    };
}
