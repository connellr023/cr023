use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectEntryProps
{
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub images: Vec<(&'static str, &'static str)>,
    pub repo_url: Option<&'static str>,
    pub site_url: Option<&'static str>
}

#[function_component(ProjectEntry)]
pub fn project_entry(ProjectEntryProps { name, description, version, images, repo_url, site_url }: &ProjectEntryProps) -> Html
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
    		<div>
    			<h3 class={"project-name"}>{name}</h3>
    			<div class={"project-version"}>{version}</div>
    		</div>
    		<p class={"project-desc"}>{description}</p>
            <button class={"image-switch-button"} onclick={prev_image}>{"<"}</button>
    		<img class={"current-image"} alt={images[*current_index].0} src={images[*current_index].1} />
            <button class={"image-switch-button"} onclick={next_image}>{">"}</button>
    		{render_option_url(repo_url, "repo-url", "no-repo-url", "This project is not open source")}
            {render_option_url(site_url, "site-url", "no-site-url", "This project does not have a website currently")}
    	</div>
    }
}

fn render_option_url(url: &Option<&str>, exists_class: &'static str, not_exists_class: &'static str, not_exists_msg: &'static str) -> Html
{
    match *url
    {
    	Some(url_val) => {
    	   html! { <div><a class={exists_class}>{url_val}</a></div> }
    	},
    	None => {
    	   html! { <div class={not_exists_class}>{not_exists_msg}</div> }
    	}
    }
}