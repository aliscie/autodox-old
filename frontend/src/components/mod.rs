// here we create general specific_components that are reusable by any app
// Don't import anything this folder from outside.

pub use avatar::Avatar;
pub use directory::CurDirectory;
pub use menu::PopOverMenu;
pub use title_bar::TitleBar;
mod avatar;
mod directory;
mod menu;
pub mod title_bar;
