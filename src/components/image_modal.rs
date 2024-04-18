use yew::prelude::*;
use crate::components::AltSrcTuple;

#[derive(Properties, PartialEq)]
pub struct ImageModalProps
{
    current_image: AltSrcTuple
}

#[function_component(ImageModal)]
pub fn image_modal(ImageModalProps { current_image }: &ImageModalProps) -> Html
{
    let hidden = use_state(|| true);
    let hidden_clone = hidden.clone();

    let toggle_hidden = Callback::from(move |_|
    {
        hidden_clone.set(!(*hidden_clone));
    });

    html!
    {
        <div class={"img-modal-wrapper"} hidden={*hidden} onclick={toggle_hidden}>
            <img alt={(*current_image).0} src={(*current_image).1} />
            <div>{"Click anywhere off the image to exit"}</div>
        </div>
    }
}