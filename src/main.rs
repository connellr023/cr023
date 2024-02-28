mod components;
mod bindings;

use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::{prelude::Closure, JsCast};
use components::{name_section::NameSection, gh_img_btn::GithubImageButton};
use web_sys::{window};
use crate::bindings::{log};

const MAIN_STYLESHEET: &str = include_str!("styles/main.css");

#[styled_component(App)]
fn app() -> Html
{
    let main_stylesheet: Style = Style::new(MAIN_STYLESHEET).unwrap();
    let window = window().unwrap();

    //let top_bar_visible = use_state(|| false);

    use_effect(move || {
        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            log("Scrolled");
        }));

        window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
            .unwrap();

        move || {
            window.remove_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    html!
    {
        <main id={"app-main"} class={main_stylesheet}>
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