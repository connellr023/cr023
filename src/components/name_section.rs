use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;
use crate::components::typer::Typer;

const STYLESHEET: &str = include_str!("../styles/name_section.css");

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub title_name: String,
    pub full_name: String
}

#[styled_component(NameSection)]
pub fn name_section(NameSectionProps { title_name, full_name }: &NameSectionProps) -> Html
{
    let stylesheet: Style = Style::new(STYLESHEET).unwrap();

    html!
    {
        <div class={stylesheet}>
            <h1 class={"title-name mono"}>{format!("<{}>", title_name)}</h1>
            <h1 class={"full-name"}><Typer word={String::from(full_name)} interval={120} /></h1>
        </div>
    }
}