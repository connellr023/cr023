use yew::prelude::*;
use crate::components::string_set::StringSet;

#[derive(Properties, PartialEq)]
pub struct SnippetProps
{
    pub object: &'static str,
    pub property: &'static str,
    pub values: Vec<&'static str>
}

#[function_component(Snippet)]
pub fn snippet(SnippetProps { object, property, values }: &SnippetProps) -> Html
{
    html!
    {
        <div class={"snippet-wrapper mono"}>
            <div class={"property"}>{format!("{}->{}", *object, *property)}</div>
            <StringSet values={values.clone()} />
        </div>
    }
}