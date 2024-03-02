use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SnippetProps
{
    pub property: &'static str,
    pub value: &'static str
}

#[function_component(Snippet)]
pub fn snippet(SnippetProps { property, value }: &SnippetProps) -> Html
{
    html!
    {
        <div class={"snippet-wrapper"}>
            <div class={"property"}><span>{">"}</span>{*property}</div>
            <div class={"value"}>{*value}</div>
        </div>
    }
}