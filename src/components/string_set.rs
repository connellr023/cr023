use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StringSetProps
{
    pub values: Vec<&'static str>
}

#[function_component(StringSet)]
pub fn string_set(StringSetProps { values }: &StringSetProps) -> Html
{
    let length: usize = values.len();
    let mut arr_counter: usize = 0;

    html!
    {
        <div class={"value"}>
            <span class={"bracket-token left"}>{"{"}</span>
            { for values.iter().map(|string| {
                arr_counter += 1;
                html!
                {
                    <>
                        <span class="str-token">{r#"""#}{*string}{r#"""#}</span>
                        { if arr_counter < length { html! { <span class="comma-token">{","}</span> } } else { html! {} } }
                    </>
                }
            }) }
            <span class={"bracket-token right"}>{"}"}</span>
        </div>
    }
}