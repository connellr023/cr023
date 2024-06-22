use crate::components::prelude::*;
use crate::components::{
    string_set::StringSet,
    animation_wrapper::AnimationWrapper,
    AltSrcTuple,
    ImgClickCallback
};

#[derive(Properties, PartialEq)]
pub struct ProjectEntryProps {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub tech_stack: &'static [&'static str],
    pub images: Option<&'static [AltSrcTuple]>,
    pub repo_url: Option<&'static str>,
    pub site_url: Option<&'static str>,
    pub update_current_image: Option<Rc<ImgClickCallback>>
}

#[function_component(ProjectEntry)]
pub fn project_entry(ProjectEntryProps {
    name,
    description,
    version,
    tech_stack,
    images,
    repo_url,
    site_url,
    update_current_image 
}: &ProjectEntryProps) -> Html
{
    let hidden = Rc::new(use_state(|| true));

    let toggle_hidden_content = Callback::from({
        let hidden = Rc::clone(&hidden);

        move |_| {
            hidden.set(!(**hidden));
        }
    });

    html! {
    	<div class={"project-entry-item"}>
            <div onclick={toggle_hidden_content} class="project-info-wrapper">
                <span class={"project-name mono"}>{name}</span>
                <span class={"project-version"}>{version}</span>
            </div>
            <AnimationWrapper reset={!(**hidden)} hidden={**hidden} class={"project-content"} animation_class={"fade-in-children"}>
                <StringSet values={*tech_stack} />
                <p class={"project-desc mono side-border"}>{description}</p>
                {render_image_content(images, update_current_image)}
                <div class={"project-links mono"}>
                    <div class={"link-entry"}>
                        <span class={"link-title"}>{"Project Repository"}</span>
                        {render_option_url(repo_url, "project-url repo-url", "no-repo-url", "This project is not open source")}
                    </div>
                    <div class={"link-entry"}>
                        <span class={"link-title"}>{"Project Deployment"}</span>
                        {render_option_url(site_url, "project-url site-url", "no-site-url", "This project is not currently deployed")}
                    </div>
                </div>
            </AnimationWrapper>
        </div>
    }
}

fn render_image_content(images: &Option<&'static [AltSrcTuple]>, callback: &Option<Rc<ImgClickCallback>>) -> Html {
    match images {
        Some(images) => {
            let current_index = Rc::new(use_state(|| 0));

            let next_image = Callback::from({
                let current_index = Rc::clone(&current_index);
                let images_len = images.len();

                move |_| {
                    current_index.set((**current_index + 1) % images_len);
                }
            });

            let prev_image = Callback::from({
                let current_index = Rc::clone(&current_index);
                let images_len = images.len();

                move |_| {
                    if **current_index == 0 {
                        current_index.set(images_len - 1);
                    }
                    else {
                        current_index.set(**current_index - 1)
                    }
                }
            });

            let current_image = images[**current_index];
            let handle_img_click = match callback {
                Some(callback) => {
                    let callback = Rc::clone(callback);

                    Callback::once(move |_|
                    {
                        callback.emit(Some(current_image));
                    })
                },
                None => {
                    Callback::from(|_| {})
                }
            };

            html! {
                <div class={"image-wrapper"}>
                    <div class={"image-content"}>
                        <button class={"image-switch-button left"} onclick={prev_image}>{"<"}</button>
                        <img onclick={handle_img_click} class={"current-image"} alt={images[**current_index].0} src={images[**current_index].1} />
                        <button class={"image-switch-button right"} onclick={next_image}>{">"}</button>
                    </div>
                    {render_carousel_index_indicator(**current_index, images.len())}
                </div>
            }
        },
        None => {
            html! {}
        }
    }
}

fn render_option_url(url: &Option<&'static str>, exists_class: &'static str, not_exists_class: &'static str, not_exists_msg: &'static str) -> Html {
    match *url {
    	Some(url_val) => {
    	   html! { <a class={exists_class} href={url_val} target={"_blank"}>{url_val}</a> }
    	},
    	None => {
    	   html! { <span class={not_exists_class}>{not_exists_msg}</span> }
    	}
    }
}

fn render_carousel_index_indicator(current_index: usize, image_count: usize) -> Html {
    html! {
        <div class={"carousel-indicator"}>
            { for (0..image_count).map(|i| {
                if i == current_index {
                    html! { <span class={"indicator-element selected"}>{"0"}</span> }
                }
                else {
                    html! { <span class={"indicator-element"}>{"o"}</span> }
                }
            }) }
        </div>
    }
}