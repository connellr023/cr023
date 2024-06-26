use crate::components::prelude::*;
use crate::components::{
    AltSrcTuple,
    ImgClickCallback
};

#[derive(Properties, PartialEq)]
pub struct ImageModalProps {
    pub current_image: Option<AltSrcTuple>,
    pub update_current_image: Rc<ImgClickCallback>
}

#[function_component(ImageModal)]
pub fn image_modal(ImageModalProps { current_image, update_current_image }: &ImageModalProps) -> Html {
    match current_image {
        Some(current_image) => {
            let hide = Callback::from({
                let update_current_image = Rc::clone(update_current_image);

                move |_| {
                    update_current_image.emit(None);
                }
            });

            html! {
                <div class={"img-modal-wrapper"} onclick={hide}>
                    <div class={"img-title mono"}>{(*current_image).0}</div>
                    <img alt={(*current_image).0} src={(*current_image).1} />
                    <div class={"prompt mono"}>{"< Click anywhere to exit >"}</div>
                </div>
            }
        },
        None => {
            html! {}
        }
    }
}