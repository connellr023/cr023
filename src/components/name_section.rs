use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;
use crate::components::typer::Typer;

const STYLESHEET: &str = include_str!("../styles/name_section.css");

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub full_name: String,
    pub title_name: String
}

#[styled_component(NameSection)]
pub fn name_section(NameSectionProps { full_name, title_name }: &NameSectionProps) -> Html
{
    let stylesheet: Style = Style::new(STYLESHEET).unwrap();

    html!
    {
        <div class={stylesheet}>
            <h1 class={"full-name"}>{full_name}</h1>
            <Typer class={"title-name mono"} word={String::from(title_name)} interval={150} />
        </div>
    }
}