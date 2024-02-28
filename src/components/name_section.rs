use yew::prelude::*;
use web_sys::{window, Window};
use wasm_bindgen::{closure::Closure, JsCast};
use stylist::Style;
use stylist::yew::styled_component;
use crate::components::typer::Typer;
use crate::bindings::log;

const STYLESHEET: &str = include_str!("../styles/name_section.css");

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub title: String,
    pub message: String
}

#[styled_component(NameSection)]
pub fn name_section(NameSectionProps { title, message }: &NameSectionProps) -> Html
{
    let stylesheet: Style = Style::new(STYLESHEET).unwrap();
    let window: Window = window().unwrap();

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
        <div class={stylesheet}>
            <h1 class={"title mono"}>{"{"}<a href="https://github.com/connellr023" target="_blank">{title}</a>{"}"}</h1>
            <i class={"message"}><Typer word={message.clone()} interval={110} /></i>
        </div>
    }
}