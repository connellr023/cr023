use yew::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use crate::bindings::log;

#[derive(Serialize, Deserialize)]
struct Committer
{
    pub name: String,
    pub email: String,
    pub date: String
}

#[derive(Serialize, Deserialize)]
struct Commit
{
    pub committer: Committer
}

#[derive(Serialize, Deserialize)]
struct CommitResponse
{
    pub commit: Commit,
    pub html_url: String
}

const API_ENDPOINT: &str = "https://api.github.com/repos/connellr023/cr023/commits/main";

#[function_component(RepoUpdates)]
pub fn repo_updates() -> Html
{
    let commit = use_state(|| None as Option<CommitResponse>);
    let commit_clone = commit.clone();

    use_effect(|| {
        if !(*commit).is_some() {
            wasm_bindgen_futures::spawn_local(async move {                
                match Request::get(&API_ENDPOINT).send().await {
                    Ok(response) => {
                        match response.json().await {
                            Ok(fetched_commit) => {
                                commit.set(fetched_commit);
                            },
                            Err(_) => {
                                log("Failed to parse API response");
                            }
                        }
                    },
                    Err(_) => {
                        log("Failed to reach API endpoint");
                    }
                }
            });
        }

        || {}
    });

    html! {
        <div class={"repo-updates-wrapper mono"}>
            <span class={"dev-credit"}>{"Connell Reffo 2024"}</span>
            <a class={"repo-license"} target={"_blank"} href={"https://opensource.org/license/mit"}>{"MIT"}</a>
            <span class={"dash"}>{"::"}</span>
            {render_repo_commit(&commit_clone)}
        </div>
    }
}

fn render_repo_commit(commit: &Option<CommitResponse>) -> Html
{
    match commit {
        Some(commit) => {
            html! {
                <>
                    <span class={"date"}>{format!("Updated On {}", commit.commit.committer.date)}</span>
                    <a class={"last-commit"} target={"_blank"} href={format!("{}", commit.html_url)}>{format!("{}...", &commit.html_url[58..65])}</a>
                </>
            }
        },
        None => {
            html! { <span class={"commit-not-loaded"}>{"Commit Content Not Loaded"}</span> }
        }
    }
}