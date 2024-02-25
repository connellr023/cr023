use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;

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
            <div class={"title-name mono"}>{title_name}</div>
        </div>
    }
}