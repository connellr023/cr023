use crate::bindings::set_timeout;
use crate::components::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TyperProps {
    pub class: Option<&'static str>,
    pub word: &'static str,
    pub interval: u32,
    pub reset: bool,
    pub start_index: Option<usize>
}

#[function_component(Typer)]
pub fn typer(TyperProps { class, word, interval, reset, start_index }: &TyperProps) -> Html {
    let start_index = if let Some(start_index) = start_index { *start_index } else { 0usize };
    let end = Rc::new(use_state(|| start_index));
    let word_ref: &str = &word;
    let len: usize = word.len();

    use_effect_with_deps({
        let end = Rc::clone(&end);
        let start_index = start_index.clone();
        let reset = reset.clone();

        move |_| {
            if **end >= len && reset {
                end.set(start_index);
            }
    
            || {}
        }
    }, reset.clone());

    use_effect_with_deps({
        let end = Rc::clone(&end);
        let interval = interval.clone();

        move |_| {
            if **end < len {
                let timeout_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    end.set(
                        **end +
                            if word_ref.chars().nth(**end).unwrap() == ' '
                            { 2 }
                            else
                            { 1 }
                    );
                }));
    
                set_timeout(&timeout_closure, interval);
    
                // Cleanup function if timeout is started
                timeout_closure.forget()
            }
    
            // Cleanup function if timer is not started
            || {}
        }
    }, **end);

    html! { <span id={format!("{}", *reset)} class={*class}>{&word[0..**end]}</span> }
}