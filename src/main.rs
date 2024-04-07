mod components;
mod bindings;

use yew::prelude::*;
use web_sys::{window, Window, Element};
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use crate::components::{
    name_section::NameSection,
    gh_img_btn::GithubImageButton,
    scroll_prompt::ScrollPrompt,
    snippet::Snippet,
    typer::Typer,
    blinker::Blinker,
    project_entry::ProjectEntry
};

#[function_component(App)]
fn app() -> Html
{
    let window: Window = window().unwrap();
    let in_view = use_state(|| true);
    let in_view_clone = in_view.clone();

    use_effect(move ||
    {
        let window_clone: Window = window.clone();
        let element: Element = window
            .document()
            .unwrap_throw()
            .get_element_by_id("name-section-wrapper")
            .unwrap_throw();

        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move ||
        {
            let window_scroll_y: f64 = window_clone.scroll_y().unwrap_throw();
            let element_scroll_height: f64 = element.scroll_height().into();

            if window_scroll_y > element_scroll_height {
                in_view_clone.set(false);
            }
            else {
                in_view_clone.set(true);
            }
        }));

        window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref()).unwrap();

        // Cleanup function for window scroll listener
        move || { window.remove_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref()).unwrap(); }
    });

    html!
    {
        <main id={"app-wrapper"} class={format!("flex-wrapper {}", if *in_view { "in-view" } else { "" })}>
            <GithubImageButton />
            <ScrollPrompt />
            <NameSection name={"connellr023"} />
            <div id={"content-wrapper"}>
                <div class={"abstract side-border"}>
                    <Typer reset={!(*in_view)} word={"I need a degree."} interval={70} />
                    <Blinker symbol={"_"} interval={450} />
                </div>
                <div id={"snippets-wrapper"} class={"mono side-border"}>
                    <Snippet property={"cr023.languages"} values={vec!["Java", "PHP", "Typescript", "Rust", "C/C++"]} />
                    <Snippet property={"cr023.frameworks"} values={vec!["React.js", "Next.js", "Vue.js", "Yew.rs", "express.js"]} />
                    <Snippet property={"cr023.testing"} values={vec!["Jest", "PHPUnit", "JUnit"]} />
                    <Snippet property={"cr023.tools"} values={vec!["git"]} />
                    <Snippet property={"cr023.location"} values={vec!["Calgary, AB"]} />
                </div>
                <h2 class={"sub-heading mono side-border"}>{"cr023"}<span>{"."}</span>{"projects"}</h2>
                <div class={"projects-wrapper"}>
                    <ProjectEntry
                        name={"Chatter"}
                        version={"v1.0.3"}
                        images={vec![("sample", "tmp.png")]}
                        url={"https://github.com/connellr023/Chatter"}
                        description={"A web based chat application that is entirely oriented around temporary sessions."}
                    />
                </div>
            </div>
            <div class={"spacer"} />
        </main>
    }
}

fn main()
{
    yew::start_app::<App>();
}