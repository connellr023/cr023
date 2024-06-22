// Portfolio Website
// Author: Connell Reffo
// Developed: 2024
#![crate_name = "cr023"]

mod components;
mod bindings;
mod prelude;

use std::rc::Rc;
use yew::prelude::*;
use web_sys::{window, Window, Element};
use wasm_bindgen::{closure::Closure, JsCast};
use prelude::*;

#[function_component(App)]
fn app() -> Html {
    let window = window().expect("Window object does not exist.");

    let current_image = Rc::new(use_state::<Option<AltSrcTuple>, _>(|| None));
    let in_view = Rc::new(use_state(|| true));

    // Use effect for handling scroll effect event
    use_effect({
        let in_view = Rc::clone(&in_view);

        move || {
            let window_clone: Window = window.clone();
            let element: Element = window
                .document()
                .unwrap()
                .get_element_by_id("name-section-wrapper")
                .unwrap();
    
            let on_scroll_callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let window_scroll_y: f64 = window_clone.scroll_y().unwrap();
                let element_scroll_height: f64 = element.scroll_height().into();
    
                if window_scroll_y > element_scroll_height {
                    in_view.set(false);
                }
                else {
                    in_view.set(true);
                }
            }));
    
            window.add_event_listener_with_callback("scroll", on_scroll_callback.as_ref().unchecked_ref()).unwrap();
    
            // Cleanup function for window scroll listener
            move || { window.remove_event_listener_with_callback("scroll", on_scroll_callback.as_ref().unchecked_ref()).unwrap(); }
        }
    });

    // Callback for updating the current image to be displayed by the image modal
    let set_image_callback = Rc::new(
        Callback::from({
            let current_image = Rc::clone(&current_image);

            move |image: Option<AltSrcTuple>| {
                current_image.set(image);
            }
        })
    );

    html! {
        <main id={"app-wrapper"} class={format!("flex-wrapper {}", if **in_view { "in-view" } else { "" })}>
            <GithubImageButton />
            <ScrollPrompt />
            <ImageModal current_image={**current_image} update_current_image={Rc::clone(&set_image_callback)} />
            <AnimationWrapper reset={**in_view} class={"section-nav mono"} animation_class={"fade-up-children-6"}>
                <a href={"#about-section"}>{"About"}</a>
                <a href={"#education-section"}>{"Education"}</a>
                <a href={"#main-projects-section"}>{"Main Projects"}</a>
            </AnimationWrapper>
            <NameSection name={"connellr023"} reset={**in_view} />
            <div id={"content-wrapper"}>
                <h3 id={"about-section"} class={"section"}><Typer reset={!(**in_view)} word={"About"} interval={140} start_index={1} /></h3>
                <div class={"abstract side-border"}>{"I am a versatile programmer with a strong command of a wide array of tools and languages. My expertise spans both object-oriented and functional programming paradigms, and I excel in testing using a variety of languages and frameworks as well as have experience working in teams with version control. Adaptable and passionate, I bring a dynamic approach to solving complex problems in software development."}</div>
                <AnimationWrapper reset={!(**in_view)} id={"snippets-wrapper"} class={"mono side-border"} animation_class={"fade-up-children-6"}>
                    <Snippet object={"cr023"} property={"languages"} values={&["Java", "PHP", "typescript", "Rust", "C/C++"] as &'static [&'static str]} />
                    <Snippet object={"cr023"} property={"frameworks"} values={&["React.js", "Next.js", "Vue.js", "Yew.rs", "express.js"] as &'static [&'static str]} />
                    <Snippet object={"cr023"} property={"databases"} values={&["MySQL", "PostgreSQL", "RedisKV", "MongoDB"] as &'static [&'static str]} />
                    <Snippet object={"cr023"} property={"testing"} values={&["Jest", "Vitest", "PHPUnit", "JUnit", "cargo test"] as &'static [&'static str]} />
                    <Snippet object={"cr023"} property={"tools"} values={&["git", "Docker", "Adobe Suite"] as &'static [&'static str]} />
                    <Snippet object={"cr023"} property={"location"} values={&["Calgary, AB"] as &'static [&'static str]} />
                </AnimationWrapper>
                <h3 id={"education-section"} class={"section"}><Typer reset={!(**in_view)} word={"Education"} interval={105} start_index={1} /></h3>
                <div style={"margin-bottom: 175px"} class={"abstract side-border"}>{"While a lot of my current skills are self taught, I am currently a Computer Science student at the University of Calgary where I have gained valuable experience working with others and made important connections."}</div>
                <h3 id={"main-projects-section"} class={"section"}><Typer reset={!(**in_view)} word={"Main Projects"} interval={80} start_index={1} /></h3>
                <div class={"abstract side-border"}>{"It is worth noting that the projects below are a select few of very many. A lot of my skills are built off of unfinished as well as private work done for school which were excellent but I do not believe belong on this portfolio."}</div>
                <div class={"projects-wrapper"}>
                    <ProjectEntry
                        name={"cr023"}
                        version={"v3"}
                        tech_stack={&["Yew.rs", "Rust", "WebAssembly"] as &'static [&'static str]}
                        description={"The website you are currently on! A portfolio to showcase my work. This project specifically is different as I used it as an introduction to the Rust programming language with its web assembly compilation feature. This was created utilizing the Yew framework which is very similar to React for creating front end web services."}
                        repo_url={"https://github.com/connellr023/cr023"}
                        site_url={"https://connellr023.github.io/cr023/"}
                    />
                    <ProjectEntry
                        name={"Oncology Quest"}
                        version={"v1.0.0"}
                        tech_stack={&["Rust", "Actix Web", "SQLx", "PostgreSQL", "Vue.js", "typescript", "Docker", "AWS"] as &'static [&'static str]}
                        images={&[
                            ("Oncology Quest Home Page Screen", "assets/oncology_quest/1.png"),
                            ("Oncology Quest Rotation Select Screen", "assets/oncology_quest/2.png"),
                            ("Oncology Quest Task Editor Screen", "assets/oncology_quest/3.png"),
                            ("Oncology Quest Task View Screen", "assets/oncology_quest/4.png"),
                            ("Oncology Quest Search Users Screen", "assets/oncology_quest/5.png"),
                            ("Oncology Quest View Progress Screen", "assets/oncology_quest/6.png")
                        ] as &'static [AltSrcTuple]}
                        repo_url={"https://github.com/connellr023/oncology-quest"}
                        site_url={"https://www.oncologyquest.net/"}
                        description={"Oncology Quest is a web app solution to the problem of tracking and managing tasks for Medical Oncology trainees. It features a task editor, task viewer, and user search functionality for admins. The app is designed to be user-friendly for both regular users and admins, with a clean and intuitive dark themed interface."}
                        update_current_image={Rc::clone(&set_image_callback)}
                    />
                    <ProjectEntry
                        name={"Chatter"}
                        version={"v1.0.3"}
                        images={&[
                            ("Chatter Home Page Screen", "assets/chatter/1.png"),
                            ("Chatter Empty Chat Screen", "assets/chatter/2.png"),
                            ("Chatter Chat Screen", "assets/chatter/3.png"),
                            ("Chatter Error Screen", "assets/chatter/4.png")
                        ] as &'static [AltSrcTuple]}
                        tech_stack={&["Vue.js", "socket.io", "express.js", "Jest", "Vitest", "typescript"] as &'static [&'static str]}
                        repo_url={"https://github.com/connellr023/Chatter"}
                        site_url={"https://chatter-lqqb.onrender.com"}
                        description={"Chatter is a web app centered around a global chat system. It features isolated chat rooms that users can connect to without requiring an account. Currently, all chat rooms are global, however there is infrastructure in place within the server the API to allow for private chat rooms in the future."}
                        update_current_image={Rc::clone(&set_image_callback)}
                    />
                    <ProjectEntry
                        name={"tensort"}
                        version={"v1.0.0"}
                        images={&[("Tensort Example Images", "assets/tensort/1.png"), ("Tensort Usage", "assets/tensort/2.png"), ("Tensort Image Recognition", "assets/tensort/3.png"), ("Tensort Example Output 2", "assets/tensort/4.png")] as &'static [AltSrcTuple]}
                        tech_stack={&["Rust", "PyTorch", "Docker"] as &'static [&'static str]}
                        repo_url={"https://github.com/connellr023/tensort"}
                        site_url={"https://github.com/connellr023/tensort"}
                        description={"A CLI tool that utilizes a ResNet convolutional neural network to recognize content in images and sort them into classes."}
                        update_current_image={Rc::clone(&set_image_callback)}
                    />
                    <ProjectEntry
                        name={"Atla"}
                        version={"v1.0.0"}
                        images={&[("Atla Welcome Screen", "assets/atla/1.png"), ("Atla Blank Map", "assets/atla/2.png"), ("Atla Add Event Map", "assets/atla/3.png"), ("Atla Open Event Screen", "assets/atla/4.png"), ("Atla View Event Screen", "assets/atla/5.png")] as &'static [AltSrcTuple]}
                        tech_stack={&["Next.js", "Vercel", "redis", "typescript"] as &'static [&'static str]}
                        repo_url={"https://github.com/connellr023/Atla"}
                        site_url={"https://atla-ch2024.vercel.app"}
                        description={"Your Hub for Volunteering Events and more, Alta (made for Calgary Hacks 2024) aims to bring the Calgarian community together by providing a centralized platform to post and view volunteering events."}
                        update_current_image={Rc::clone(&set_image_callback)}
                    />
                    <ProjectEntry
                        name={"Crumble"}
                        version={"v1.0.0"}
                        images={&[("Crumble Start Screen", "assets/crumble/1.png"), ("Crumble Queue Screen", "assets/crumble/2.png"), ("Crumble Game Screen 1", "assets/crumble/3.png"), ("Crumble Game Screen 2", "assets/crumble/4.png"), ("Crumble Falling Player", "assets/crumble/5.png"), ("Crumble Win Screen", "assets/crumble/6.png")] as &'static [AltSrcTuple]}
                        tech_stack={&["p5.js", "typescript", "jquery"] as &'static [&'static str]}
                        repo_url={"https://github.com/connellr023/Crumble"}
                        site_url={"https://crumble-b4fq.onrender.com"}
                        description={"A top-down view web game where you must battle another opponent with a rocket launcher on a crumbling map."}
                        update_current_image={Rc::clone(&set_image_callback)}
                    />
                    <ProjectEntry
                        name={"gratis"}
                        version={"v1.0.1"}
                        tech_stack={&["PHP 8.3", "PHPUnit"] as &'static [&'static str]}
                        repo_url={"https://github.com/connellr023/gratis"}
                        site_url={"https://packagist.org/packages/connell/gratis"}
                        description={"Gratis is a versatile framework designed to promote the separation of concerns, fostering scalable code practices by encapsulating logic within handlers. Primarily tailored for creating robust and scalable APIs that follow the CRUD lifecycle, the framework follows a REST-like architectural style. It allows form seamless interactions with SQL databases, providing a structured and efficient foundation for building web applications."}
                    />
                </div>
                <RepoUpdates />
            </div>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}