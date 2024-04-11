use yew::prelude::*;
use crate::components::snippet::Snippet;

#[derive(Properties, PartialEq)]
pub struct ProjectEntryProps
{
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub images: Vec<(&'static str, &'static str)>,
    pub contributers: Vec<&'static str>,
    pub repo_url: Option<&'static str>,
    pub site_url: Option<&'static str>
}

#[function_component(ProjectEntry)]
pub fn project_entry(ProjectEntryProps { name, description, version, images, contributers, repo_url, site_url }: &ProjectEntryProps) -> Html
{
    let current_index = use_state(|| 0);
    let images_len = images.len().clone();

    let current_index_clone = current_index.clone();
    let next_image = Callback::from(move |_|
    {
        current_index_clone.set((*current_index_clone + 1) % images_len);
    });

    let current_index_clone = current_index.clone();
    let prev_image = Callback::from(move |_|
    {
        if *current_index_clone == 0 {
            current_index_clone.set(images_len - 1);
        }
        else {
            current_index_clone.set(*current_index_clone - 1)
        }
    });

    html!
    {
    	<div class={"project-entry-item"}>
            <h3 class={"project-name sub-heading mono"}>
                {name}
                <span>{"::"}</span>
                <span class={"project-version"}>{version}</span>
            </h3>
            <Snippet property={"contributers"} values={vec!["Connell Reffo"]} />
    		<p class={"project-desc mono side-border"}>{description}</p>
            <div class={"image-wrapper"}>
                <button class={"image-switch-button left"} onclick={prev_image}>{"<"}</button>
                <img class={"current-image"} alt={images[*current_index].0} src={images[*current_index].1} />
                <button class={"image-switch-button right"} onclick={next_image}>{">"}</button>
                {render_carousel_index_indicator(*current_index, images_len)}
            </div>
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
        </div>
    }
}

fn render_option_url(url: &Option<&'static str>, exists_class: &'static str, not_exists_class: &'static str, not_exists_msg: &'static str) -> Html
{
    match *url
    {
    	Some(url_val) => {
    	   html! { <a class={exists_class} href={url_val} target={"_blank"}>{url_val}</a> }
    	},
    	None => {
    	   html! { <span class={not_exists_class}>{not_exists_msg}</span> }
    	}
    }
}

fn render_carousel_index_indicator(current_index: usize, image_count: usize) -> Html
{
    html!
    {
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