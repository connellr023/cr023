use crate::components::string_set::StringSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SnippetProps
{
    pub object: &'static str,
    pub property: &'static str,
    pub values: &'static [&'static str]
}

#[function_component(Snippet)]
pub fn snippet(SnippetProps { object, property, values }: &SnippetProps) -> Html {
    html! {
        <div class={"snippet-wrapper mono"}>
            <div class={"property"}>{*object}<span>{"->"}</span>{*property}</div>
            <StringSet values={values.to_owned()} />
        </div>
    }
}