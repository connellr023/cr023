use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;
use crate::components::typer::Typer;

const STYLESHEET: &str = include_str!("../styles/name_section.css");

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub title: String,
    pub message: String
}

#[styled_component(NameSection)]
pub fn name_section(NameSectionProps { title, message }: &NameSectionProps) -> Html
{
    let stylesheet: Style = Style::new(STYLESHEET).unwrap();

    html!
    {
        <div class={stylesheet}>
            <h1 class={"title mono"}>{format!("<{}>", title)}</h1>
            <i class={"message"}><Typer word={message.clone()} interval={110} /></i>
        </div>
    }
}