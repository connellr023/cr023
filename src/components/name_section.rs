use yew::prelude::*;
use crate::components::typer::Typer;

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub title: String,
    pub message: String
}

#[function_component(NameSection)]
pub fn name_section(NameSectionProps { title, message }: &NameSectionProps) -> Html
{
    html!
    {
        <div id="name-section-wrapper">
            <h1 class={"title mono"}>{"{"}<a href="https://github.com/connellr023" target="_blank">{title}</a>{"}"}</h1>
            <i class={"message"}><Typer word={message.clone()} interval={110} /></i>
        </div>
    }
}