mod tree;

#[macro_export]
macro_rules! css_file_macro {

    ($path: expr) => {
        {
        use stylist::{style, yew::styled_component, Style};
        let styles = std::include_str!($path);
        Style::new(styles).unwrap()
        }
    }
}

use lazy_static::lazy_static;
pub use tree::Tree;
cfg_if::cfg_if! {
    if #[cfg(feature = "autodox_assets")]{
        mod invoke;
        pub use invoke::{invoke, invoke_async};
    }
}

// lazy_static! {
//     pub static ref IS_WEB: bool = {
//         #[cfg(feature = "web")] {
//             true
//         }
//         false
//     };
// }
