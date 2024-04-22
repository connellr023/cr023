use yew::prelude::*;
use wasm_bindgen::prelude::Closure;
use crate::bindings::set_timeout;

#[derive(Properties, PartialEq)]
pub struct BlinkerProps
{
    pub class: Option<&'static str>,
    pub symbol: &'static str,
    pub interval: u32
}

#[function_component(Blinker)]
pub fn blinker(BlinkerProps { class, symbol, interval }: &BlinkerProps) -> Html
{
    let visible = use_state(|| true);
    let visible_clone = visible.clone();
    let interval_clone: u32 = interval.clone();

    use_effect_with_deps(move |_| {
        let timeout_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            visible_clone.set(!(*visible_clone));
        }));

        set_timeout(&timeout_closure, interval_clone);

        // Timeout cleanup function
        || { timeout_closure.forget() }
    }, visible.clone());

    html! { <span class={*class}>{if *visible { &symbol } else { "" }}</span> }
}