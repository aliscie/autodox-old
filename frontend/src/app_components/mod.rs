// Note you can import from app_components to app_components but not the apposite.
// Note: you can do data handle in app_components but you cannot do it in app_components
// Note: app_components are reusable but app_components are not.

mod file_component;

pub use file_component::FileComponent;

mod title_avatar_component;

pub use title_avatar_component::TitleAvatarComponent;

mod page_option_component;

pub use page_option_component::PageOptions;

mod download_component;
pub use download_component::Download;


mod search_filters;
pub use search_filters::SearchFiltes;


mod files_category;
pub use files_category::ButtonsGroup;

mod markdown_button;
pub use markdown_button::Markdown;
