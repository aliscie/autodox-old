// Note you can import from app_components to app_components but not the apposite.
// Note: you can do data handle in app_components but you cannot do it in app_components
// Note: app_components are reusable but app_components are not.

pub use download_component::Download;
pub use file_component::FileComponent;
pub use files_category::ButtonsGroup;
pub use markdown_button::Markdown;
pub use page_option_component::PageOptions;
pub use search_filters::SearchFiltes;
pub use title_avatar_component::TitleAvatarComponent;
pub use save_button::SaveButton;

mod file_component;

mod title_avatar_component;

mod page_option_component;

mod download_component;
mod search_filters;
mod files_category;
mod markdown_button;
mod save_button;
