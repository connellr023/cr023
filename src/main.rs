use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;

const MAIN_STYLESHEET: &str = include_str!("styles/main.css");

#[styled_component(App)]
fn app() -> Html {
    let main_stylesheet: Style = Style::new(MAIN_STYLESHEET).unwrap();

    html! {
        <main class={main_stylesheet}>
            <h1>{"Test Page!"}</h1>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}