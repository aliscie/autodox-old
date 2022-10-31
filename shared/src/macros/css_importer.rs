#[macro_export]
macro_rules! css_file_macro {
    ($path: expr) => {{
        use stylist::{style, yew::styled_component, Style};
        let styles = std::include_str!($path);
        Style::new(styles).unwrap()
    }};
}
