use crate::components::prelude::*;
use crate::bindings::set_timeout;

#[derive(Properties, PartialEq)]
pub struct BlinkerProps {
    pub class: Option<&'static str>,
    pub symbol: &'static str,
    pub interval: u32
}

#[function_component(Blinker)]
pub fn blinker(BlinkerProps { class, symbol, interval }: &BlinkerProps) -> Html {
    let visible = Rc::new(use_state(|| true));
    
    use_effect_with_deps({
        let visible = Rc::clone(&visible);
        let interval = interval.clone();

        move |_| {
            let timeout_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                visible.set(!(**visible));
            }));
    
            set_timeout(&timeout_closure, interval);
    
            // Timeout cleanup function
            || { timeout_closure.forget() }
        }
    }, visible.clone());

    html! { <span class={*class}>{if **visible { &symbol } else { "" }}</span> }
}