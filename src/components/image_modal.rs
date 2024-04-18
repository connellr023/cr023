use yew::prelude::*;
use crate::components::AltSrcTuple;

#[derive(Properties, PartialEq)]
pub struct ImageModalProps
{
    pub current_image: AltSrcTuple
}

#[function_component(ImageModal)]
pub fn image_modal(ImageModalProps { current_image }: &ImageModalProps) -> Html
{
    let hidden = use_state(|| true);
    let hidden_clone = hidden.clone();

    use_effect_with_deps(move |_|
    {
        hidden_clone.set(false);
        
        || {}
    }, current_image.clone());

    html!
    {
        <div class={"img-modal-wrapper"} hidden={*hidden} onclick={move |_| hidden.set(true)}>
            <img alt={(*current_image).0} src={(*current_image).1} />
            <div class={"mono"}>{"< Click anywhere off the image to exit >"}</div>
        </div>
    }
}