use Class::*;
#[allow(unused)]
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(Deserialize)]
#[allow(unused)]
struct Skill {
    name: String,
    src: String,
    modifier: String,
    tag: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Job {
    name: String,
    src: String,
    class: Class,
    #[allow(unused)]
    branch: String,
    #[allow(unused)]
    skills: Option<Vec<Skill>>,
}

#[derive(Clone, Copy, Deserialize, EnumIter, Eq, Hash, PartialEq)]
enum Class {
    Warrior,
    Magician,
    Bowman,
    Thief,
    Pirate,
}

impl Class {
    fn as_string(&self) -> String {
        match self {
            Warrior => "전사",
            Magician => "마법사",
            Bowman => "궁수",
            Thief => "도적",
            Pirate => "해적",
        }
        .to_owned()
    }

    fn button_style(&self) -> String {
        match self {
            Warrior => "btn-secondary",
            Magician => "btn-info",
            Bowman => "btn-accent",
            Thief => "btn-primary",
            Pirate => "btn-neutral",
        }
        .to_owned()
    }
}

#[autoprops]
#[function_component]
fn JobCard(job_name: &AttrValue, image_name: &AttrValue) -> Html {
    let src = format!("assets/jobs/{image_name}.png");

    html! {
        <div
            class={"card w-32 bg-base-200 border-3 items-center rounded-box overflow-hidden shadow-sm"}
        >
            <img class={"bg-primary"} {src} />
            <div class={"card-body"}>
                <h2 class={"card-title text-sm whitespace-nowrap h-1"}>
                    { job_name }
                </h2>
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let theme_controller = {
        const THEME_STORAGE_KEY: &str = "theme";
        const THEME_DEFAULT_VALUE: &str = "default";

        #[derive(Clone, Deserialize)]
        struct Theme {
            value: String,
            name: String,
        }

        let initial_theme = LocalStorage::get::<String>(THEME_STORAGE_KEY)
            .unwrap_or(THEME_DEFAULT_VALUE.to_owned());

        let theme_state = use_state(|| initial_theme);

        let theme_item = |theme: &Theme| -> Html {
            let on_theme_change = {
                let theme_state = theme_state.clone();
                let theme_value = theme.value.clone();

                move |_: Event| {
                    theme_state.set(theme_value.clone());
                    LocalStorage::set(THEME_STORAGE_KEY, theme_value.clone()).unwrap();
                }
            };

            html! {
                <li key={theme.value.clone()}>
                    <input
                        type={"radio"}
                        name={"theme-dropdown"}
                        class={"theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"}
                        aria-label={theme.name.clone()}
                        value={theme.value.clone()}
                        checked={theme.value == *theme_state.clone()}
                        onchange={on_theme_change}
                    />
                </li>
            }
        };

        let theme_data = include_str!("theme.yaml");
        let themes: Vec<Theme> = serde_yaml::from_str(theme_data).unwrap();
        let theme_items: Html = themes.into_iter().map(|theme| theme_item(&theme)).collect();

        html! {
            <div class={"dropdown mb-72"}>
                <div tabindex={"0"} role={"button"} class={"btn m-1"}>
                    { "테마" }
                    <svg
                        width={"12px"}
                        height={"12px"}
                        class={"inline-block h-2 w-2 fill-current opacity-60"}
                        xmlns={"http://www.w3.org/2000/svg"}
                        viewBox={"0 0 2048 2048"}
                    >
                        <path d={"M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"} />
                    </svg>
                </div>
                <ul
                    tabindex={"0"}
                    class={"dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl"}
                >
                    { theme_items }
                </ul>
            </div>
        }
    };

    let class_handle = use_state(|| Warrior);

    let class_button_container = {
        let class_buttons: Vec<Html> = Class::iter()
            .map(|class| {
                let styles = format!("btn {}", class.button_style());

                let onclick = {
                    let class_handle = class_handle.clone();

                    move |_| {
                        class_handle.set(class);
                    }
                };

                html! {
                    <button class={styles} {onclick}>
                        { class.as_string() }
                    </button>
                }
            })
            .collect();

        html! {
            <div class={"flex justify-center"}>
                <div class={"grid grid-cols-5 gap-6"}>
                    { for class_buttons }
                </div>
            </div>
        }
    };

    let job_cards_map = {
        let job_data = include_str!("job_data.yaml");
        let jobs: Vec<Job> = serde_yaml::from_str(job_data).unwrap();

        jobs.iter()
            .map(|job| {
                let job_name = job.name.clone();
                let image_name = job.src.clone();

                (job.class.clone(), html! { <JobCard {job_name} {image_name} /> })
            })
            .fold(HashMap::new(), |mut map, (class, card)| {
                map.entry(class).or_insert_with(Vec::new).push(card);
                map
            })
    };

    let class = *class_handle;

    let job_card_container = html! {
        <div class={"flex justify-center"}>
            <div class={"grid grid-cols-5 gap-4"}>
                { for job_cards_map[&class].clone() }
            </div>
        </div>
    };

    html! {
        <div>
            <div class={"absolute right-16"}>
                { theme_controller }
            </div>
            <div class={"mt-4 grid grid-cols-1 gap-4"}>
                { class_button_container }
                { job_card_container }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
