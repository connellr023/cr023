use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectEntryProps
{
	pub name: &'static str,
	pub description: &'static str,
	pub version: &'static str,
	pub images: Vec<(&'static str, &'static str)>,
	pub url: Option<&'static str>
}

#[function_component(ProjectEntry)]
pub fn project_entry(ProjectEntryProps { name, description, version, images, url }: &ProjectEntryProps) -> Html
{
	html!
	{
		<div class={"project-entry-item"}>
			<div>
				<h3 class={"project-name"}>{name}</h3>
				<div class={"project-version"}>{version}</div>
			</div>
			<p class={"project-desc"}>{description}</p>
			<div class={"images-wrapper"}>
				<img alt={images[0].0} src={images[0].1} />
			</div>
			{render_url(url)}
		</div>
	}
}

fn render_url(url: &Option<&str>) -> Html
{
	match *url
	{
		Some(url_val) => {
			html! { <a class={"project-url"}>{url_val}</a> }
		},
		None => {
			html! { <span class={"no-project-url"}>{"This project is not open source"}</span> }
		}
	}
}