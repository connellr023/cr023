use yew::prelude::*;
use crate::components::{typer::Typer, blinker::Blinker};

#[derive(Properties, PartialEq)]
pub struct NameSectionProps
{
    pub name: &'static str,
    pub reset: bool
}

#[function_component(NameSection)]
pub fn name_section(NameSectionProps { name, reset }: &NameSectionProps) -> Html
{
    html!
    {
        <div id="name-section-wrapper">
            <h1 class={"title mono"}>
                <span class={"prefix"}>{"~$"}</span>
                <div class={"name-wrapper"}>
                    <Typer reset={*reset} class={"name"} word={*name} interval={140} />
                    <Blinker symbol={"_"} interval={450} />
                </div>
            </h1>
        </div>
    }
}