// here we create general specific_components that are reusable by anyapp
// Don't import anything this folder from outside.

pub use avatar::Avatar;
pub use directory::CurrDirectory;

pub use menu::PopOverMenu;
pub use title_bar::TitleBar;


pub mod title_bar;

mod avatar;
mod directory;
mod menu;
