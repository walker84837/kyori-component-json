//! Macros for creating components.

#[doc(hidden)]
#[macro_export]
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
macro_rules! __hover_event_from_snake {
    (show_text, { $($body:tt)* }) => { $crate::HoverEvent::ShowText { value: $($body)* } };
    (show_item, { $($body:tt)* }) => { $crate::HoverEvent::ShowItem { $($body)* } };
    (show_entity, { $($body:tt)* }) => { $crate::HoverEvent::ShowEntity { $($body)* } };
}

#[macro_export]
/// Creates a component in a declarative way.
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
    (text: $text:expr) => {
        $crate::Component::text($text)
    };

    (text: $text:expr, { $($body:tt)* }) => {
        {
            let component = $crate::Component::text($text);
            component!(@munch component, $($body)*)
        }
    };

    // Muncher syntax
    (@munch $comp:ident, ) => { $comp };

    // Munch color: yellow
    (@munch $comp:ident, color: $color:ident, $($rest:tt)*) => {
        {
            let comp = $comp.color(Some($crate::Color::Named(stringify!($color).parse().unwrap())));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, color: $color:ident) => {
        $comp.color(Some($crate::Color::Named(stringify!($color).parse().unwrap())))
    };

    // Munch color: #FFFFFF
    (@munch $comp:ident, color: #$hex:literal, $($rest:tt)*) => {
        {
            let comp = $comp.color(Some(stringify!(#$hex).replace(" ", "").parse().unwrap()));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, color: #$hex:literal) => {
        $comp.color(Some(stringify!(#$hex).replace(" ", "").parse().unwrap()))
    };

    // Munch decoration: bold & true
    (@munch $comp:ident, decoration: $deco:ident & $state:expr, $($rest:tt)*) => {
        {
            let deco = stringify!($deco).parse().unwrap();
            let comp = $comp.decoration(deco, Some($state));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, decoration: $deco:ident & $state:expr) => {
        {
            let deco = stringify!($deco).parse().unwrap();
            $comp.decoration(deco, Some($state))
        }
    };

    // Munch font: "uniform"
    (@munch $comp:ident, font: $value:literal, $($rest:tt)*) => {
        {
            let comp = $comp.font(Some($value.to_string()));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, font: $value:literal) => {
        $comp.font(Some($value.to_string()))
    };

    // Munch insertion: "text"
    (@munch $comp:ident, insertion: $value:literal, $($rest:tt)*) => {
        {
            let comp = $comp.insertion(Some($value.to_string()));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, insertion: $value:literal) => {
        $comp.insertion(Some($value.to_string()))
    };

    // Munch click_event: run_command { ... }
    (@munch $comp:ident, click_event: $type:ident { $($body:tt)* }, $($rest:tt)*) => {
        {
            let event = $crate::__click_event_from_snake!($type, { $($body)* });
            let comp = $comp.click_event(Some(event));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, click_event: $type:ident { $($body:tt)* }) => {
        {
            let event = $crate::__click_event_from_snake!($type, { $($body)* });
            $comp.click_event(Some(event))
        }
    };

    // Munch hover_event: show_text { ... }
    (@munch $comp:ident, hover_event: $type:ident { $($body:tt)* }, $($rest:tt)*) => {
        {
            let event = $crate::__hover_event_from_snake!($type, { $($body)* });
            let comp = $comp.hover_event(Some(event));
            component!(@munch comp, $($rest)*)
        }
    };

    (@munch $comp:ident, hover_event: $type:ident { $($body:tt)* }) => {
        {
            let event = $crate::__hover_event_from_snake!($type, { $($body)* });
            $comp.hover_event(Some(event))
        }
    };

    // Munch other fields (legacy value format)
    (@munch $comp:ident, $field:ident : ($($value:expr),*), $($rest:tt)*) => {
        {
            let comp = $comp.$field($($value),*);
            component!(@munch comp, $($rest)*)
        }
    };
    (@munch $comp:ident, $field:ident : ($($value:expr),*)) => {
        $comp.$field($($value),*)
    };
}
