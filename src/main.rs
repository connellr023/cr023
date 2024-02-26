mod components;
mod bindings;

use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;
use components::{name_section::NameSection, gh_img_btn::GithubImageButton};

const MAIN_STYLESHEET: &str = include_str!("styles/main.css");

#[styled_component(App)]
fn app() -> Html
{
    let main_stylesheet: Style = Style::new(MAIN_STYLESHEET).unwrap();

    html!
    {
        <main class={main_stylesheet}>
            <div class={"app-wrapper"}>
                <NameSection
                    full_name={"Connell Reffo"}
                    title_name={"connellr023"}
                />
                <GithubImageButton />
            </div>
        </main>
    }
}

fn main()
{
    yew::start_app::<App>();
}