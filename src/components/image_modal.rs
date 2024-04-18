use yew::prelude::*;
use crate::components::{
    AltSrcTuple,
    ImgClickCallback
};

#[derive(Properties, PartialEq)]
pub struct ImageModalProps
{
    pub current_image: Option<AltSrcTuple>,
    pub update_current_image: ImgClickCallback
}

#[function_component(ImageModal)]
pub fn image_modal(ImageModalProps { current_image, update_current_image }: &ImageModalProps) -> Html
{
    match current_image
    {
        Some(current_image) => {
            let update_current_image_clone = update_current_image.clone();
            let hide = Callback::from(move |_|
            {
                update_current_image_clone.emit(None);
            });

            html!
            {
                <div class={"img-modal-wrapper"} onclick={hide}>
                    <img alt={(*current_image).0} src={(*current_image).1} />
                    <div class={"mono"}>{"< Click anywhere to exit >"}</div>
                </div>
            }
        },
        None => {
            html! {}
        }
    }
}