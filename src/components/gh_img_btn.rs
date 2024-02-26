use yew::prelude::*;
use stylist::Style;

const STYLESHEET: &str = include_str!("../styles/gh_img_btn.css");

#[function_component(GithubImageButton)]
pub fn gh_img_btn() -> Html
{
    let stylesheet: Style = Style::new(STYLESHEET).unwrap();

    html!
    {
        <a class={stylesheet} title={"Project Repository"} href="https://github.com/connellr023/portfolio" target="_blank">
            <img alt="GitHub logo" src="assets/github_logo.png" />
        </a>
    }
}