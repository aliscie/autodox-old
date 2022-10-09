extern crate uuid;

use lazy_static::lazy_static;
pub use tree::Tree;

mod tree;
pub mod schema;

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



#[macro_export]
macro_rules! log {

    ($input: expr) => {
        {
        use web_sys::console::{log_1};
        let dir =  std::file!();
        let line = std::line!();
        log_1(&format!("{:#?},line:{:#?}-----> {:#?}",dir,line, $input).into());
        }
    }
}


cfg_if::cfg_if! {
    if #[cfg(feature = "frontend")]{
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
