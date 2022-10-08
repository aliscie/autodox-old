// here we create general components that are reusable by anyapp
// Don't import anything this folder from outside.

mod files;
pub use files::*;

mod main;
pub use main::*;