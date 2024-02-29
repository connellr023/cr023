mod components;
mod bindings;

use yew::prelude::*;
use web_sys::{window, Window, Element};
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use crate::components::{name_section::NameSection, gh_img_btn::GithubImageButton};

#[function_component(App)]
fn app() -> Html
{
    let window: Window = window().unwrap();
    let in_view = use_state(|| true);
    let in_view_clone = in_view.clone();

    use_effect(move || {
        let window_clone: Window = window.clone();
        let element: Element = window
            .document()
            .unwrap_throw()
            .get_element_by_id("name-section-wrapper")
            .unwrap_throw();

        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let window_scroll_y: f64 = window_clone.scroll_y().unwrap_throw();
            let element_scroll_height: f64 = element.scroll_height().into();

            if window_scroll_y > element_scroll_height {
                in_view_clone.set(false);
            }
            else {
                in_view_clone.set(true);
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
        <main id="app-wrapper" class={if *in_view { "in-view" } else { "" }}>
            <GithubImageButton />
            <NameSection
                title={"connellr023"}
                message={"I need a degree."}
            />
            <div class="spacer"></div>
        </main>
    }
}

fn main()
{
    yew::start_app::<App>();
}