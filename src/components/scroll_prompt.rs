use yew::prelude::*;

#[function_component(ScrollPrompt)]
pub fn scroll_prompt() -> Html {
    html! {
        <div id="scroll-prompt-wrapper">
            <span class="scroll-prompt mono">{"< scroll down >"}</span>
        </div>
    }
}