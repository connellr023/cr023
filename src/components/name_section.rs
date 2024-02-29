use yew::prelude::*;
use web_sys::{window, Window, Element};
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
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

    use_effect(move || {
        let element: Element = window
            .document()
            .unwrap_throw()
            .get_element_by_id("name-section-wrapper")
            .unwrap_throw();

        let window_clone: Window = window.clone();

        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let window_scroll_y: f64 = window_clone.scroll_y().unwrap_throw();
            let element_scroll_height: f64 = element.scroll_height().into();

            if window_scroll_y > element_scroll_height {
                log("Scrolled");
            }
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
        <div id="name-section-wrapper" class={stylesheet}>
            <h1 class={"title mono"}>{"{"}<a href="https://github.com/connellr023" target="_blank">{title}</a>{"}"}</h1>
            <i class={"message"}><Typer word={message.clone()} interval={110} /></i>
        </div>
    }
}