use yew::prelude::*;
use crate::components::{typer::Typer, blinker::Blinker};

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub name: &'static str
}

#[function_component(NameSection)]
pub fn name_section(NameSectionProps { name }: &NameSectionProps) -> Html
{
    html!
    {
        <div id="name-section-wrapper">
            <h1 class={"title mono"}>
                <span class={"prefix"}>{"~$"}</span>
                <Typer reset={false} class={"name"} word={*name} interval={140} />
                <Blinker symbol={"_"} interval={450} />
            </h1>
        </div>
    }
}