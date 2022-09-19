// Note you can import from components to app_components but not the apposite.
// Note: you can do data handle in app_components but you cannot do it in components
// Note: components are reusable but app_components are not.

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

