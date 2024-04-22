use yew::prelude::*;

#[function_component(GithubImageButton)]
pub fn gh_img_btn() -> Html
{
    html! {
        <a class="gh-a" title={"My GitHub Page"} href="https://github.com/connellr023" target="_blank">
            <img alt="GitHub logo" src="assets/github_logo.png" />
        </a>
    }
}