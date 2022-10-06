// here we create general components that are reusable by anyapp
// Don't import anything this folder from outside.

pub mod get_github_data;
pub mod files;
mod handle_data;

pub use handle_data::backends;
