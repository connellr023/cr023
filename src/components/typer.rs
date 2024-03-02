use yew::prelude::*;
use wasm_bindgen::prelude::Closure;
use crate::bindings::set_timeout;

#[derive(Properties, PartialEq)]
pub struct TyperProps
{
    pub class: Option<&'static str>,
    pub word: &'static str,
    pub interval: u32
}

#[function_component(Typer)]
pub fn typer(TyperProps { class, word, interval }: &TyperProps) -> Html
{
    let end = use_state(|| 0);
    let end_clone = end.clone();
    let word_ptr: &str = *word;
    let interval_clone: u32 = interval.clone();
    let len: usize = word.len();

    use_effect_with_deps(move |_| {
        if *end_clone < len {
            let timeout_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                end_clone.set(
                    *end_clone +
                        if word_ptr.chars().nth(*end_clone).unwrap() == ' '
                        { 2 }
                        else
                        { 1 }
                );
            }));

            set_timeout(&timeout_closure, interval_clone);

            (|| timeout_closure.forget())()
        }

        || {}
    }, end.clone());

    html!
    {
        <span class={*class}>{&word[0..*end]}</span>
    }
}