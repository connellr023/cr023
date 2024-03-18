use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SnippetProps
{
    pub property: &'static str,
    pub values: Vec<&'static str>
}

#[function_component(Snippet)]
pub fn snippet(SnippetProps { property, values }: &SnippetProps) -> Html
{
    let length: usize = values.len();
    let mut arr_counter: usize = 0;

    html!
    {
        <div class={"snippet-wrapper"}>
            <div class={"property"}><span>{">"}</span>{*property}</div>
            <div class={"value"}>
                <span class={"bracket-token left"}>{"{"}</span>
                    { for values.iter().map(|string| {
                        arr_counter += 1;
                        html!
                        {
                            <>
                                <span class="str-token">{r#"""#}{*string}{r#"""#}</span>
                                { if arr_counter < length { html! { <span class="comma-token">{","}</span> } } else {html! {}} }
                            </>
                        }
                    }) }
                <span class={"bracket-token right"}>{"}"}</span>
            </div>
        </div>
    }
}