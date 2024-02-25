use yew::prelude::*;
use stylist::style;
use stylist::yew::styled_component;

#[styled_component(App)]
fn app() -> Html {
    let stylesheet = style!(
        r#"
            color: red;
        "#
    )
    .unwrap();

    html! {
        <h1 class={stylesheet}>{"Test Page!"}</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}