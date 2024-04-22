use yew::prelude::*;
use wasm_bindgen::prelude::Closure;
use crate::bindings::set_timeout;

#[derive(Properties, PartialEq)]
pub struct TyperProps
{
    pub class: Option<&'static str>,
    pub word: &'static str,
    pub interval: u32,
    pub reset: bool,
    pub start_index: Option<usize>
}

#[function_component(Typer)]
pub fn typer(TyperProps { class, word, interval, reset, start_index }: &TyperProps) -> Html
{
    let start_index: usize = if let Some(start_index) = start_index { *start_index } else { 0usize };
    let end = use_state(|| start_index);
    let end_clone = end.clone();
    let reset_clone: bool = reset.clone();
    let word_ref: &str = &word;
    let interval_clone: u32 = interval.clone();
    let len: usize = word.len();
    let len_clone: usize = word.len();

    use_effect_with_deps(move |_| {
        if *end_clone >= len_clone && reset_clone {
            end_clone.set(start_index);
        }

        || {}
    }, reset.clone());

    let end_clone = end.clone();

    use_effect_with_deps(move |_| {
        if *end_clone < len {
            let timeout_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                end_clone.set(
                    *end_clone +
                        if word_ref.chars().nth(*end_clone).unwrap() == ' '
                        { 2 }
                        else
                        { 1 }
                );
            }));

            set_timeout(&timeout_closure, interval_clone);

            // Cleanup function if timeout is started
            timeout_closure.forget()
        }

        // Cleanup function if timer is not started
        || {}
    }, end.clone());

    html! { <span id={format!("{}", *reset)} class={*class}>{&word[0..*end]}</span> }
}