mod components;
mod bindings;

use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::{name_section::NameSection, gh_img_btn::GithubImageButton};

const MAIN_STYLESHEET: &str = include_str!("styles/main.css");

#[styled_component(App)]
fn app() -> Html
{
    let main_stylesheet: Style = Style::new(MAIN_STYLESHEET).unwrap();

    html!
    {
        <main class={main_stylesheet}>
            <div class={"app-wrapper"}>
                <GithubImageButton />
                <NameSection
                    title={"connellr023"}
                    message={"I need a degree."}
                />
                <div class="spacer"></div>
            </div>
        </main>
    }
}

fn main()
{
    yew::start_app::<App>();
}