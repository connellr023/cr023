pub type AltSrcTuple = (&'static str, &'static str);
pub type ImgClickCallback = yew::prelude::Callback<Option<AltSrcTuple>>;

pub mod name_section;
pub mod typer;
pub mod github_image_button;
pub mod blinker;
pub mod scroll_prompt;
pub mod snippet;
pub mod project_entry;
pub mod string_set;
pub mod animation_wrapper;
pub mod repo_updates;
pub mod image_modal;

mod prelude;