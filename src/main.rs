use Class::*;
#[allow(unused)]
use gloo_console::log;
use serde::Deserialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::prelude::*;

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
    #[allow(unused)]
    skills: Option<Vec<Skill>>,
}

#[derive(Deserialize, Eq, PartialEq, Hash, Clone, EnumIter)]
enum Class {
    Warrior,
    Magician,
    Bowman,
    Thief,
    Pirate,
}

impl Class {
    fn as_str(&self) -> &str {
        match self {
            Warrior => "전사",
            Magician => "마법사",
            Bowman => "궁수",
            Thief => "도적",
            Pirate => "해적",
        }
    }

    fn button_style(&self) -> &str {
        match self {
            Warrior => "btn-secondary",
            Magician => "btn-info",
            Bowman => "btn-accent",
            Thief => "btn-primary",
            Pirate => "btn-neutral",
        }
    }
}

fn generate_job_card(job_name: &str, image_name: &str) -> Html {
    let image_source = format!("assets/jobs/{image_name}.png");

    html! {
        <div class={"card w-32 bg-base-200 border-3 items-center rounded-box overflow-hidden shadow-sm"}>
            <img class={"bg-primary"} src={image_source} />
            <div class={"card-body"}>
                <h2 class={"card-title text-sm whitespace-nowrap h-1"}>
                    {job_name}
                </h2>
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let class_button_container = {
        let class_buttons: Vec<Html> = Class::iter()
            .map(|class| {
                let styles = format!("btn {}", class.button_style());

                html! {
                    <button class={styles}>
                        {class.as_str()}
                    </button>
                }
            })
            .collect();

        html! {
            <div class={"flex justify-center"}>
                <div class={"grid grid-cols-5 gap-6"}>
                    {for class_buttons}
                </div>
            </div>
        }
    };

    let class_data = include_str!("class_data.yaml");
    let classes: HashMap<Class, Vec<Job>> = serde_yaml::from_str(class_data).unwrap();
    let mut job_cards_map = HashMap::<Class, Vec<Html>>::new();

    for (class, jobs) in &classes {
        let mut job_cards = vec![];

        for job in jobs {
            let job_card = generate_job_card(&job.name, &job.src);
            job_cards.push(job_card);
        }

        job_cards_map.insert(class.clone(), job_cards);
    }

    let job_card_container: Html = html! {
        <div class={"flex justify-center"}>
            <div class={"grid grid-cols-5 gap-4"}>
                {for job_cards_map[&Warrior].clone()}
            </div>
        </div>
    };

    html! {
        <div class={"mt-4 grid grid-cols-1 gap-4"}>
            {class_button_container}
            {job_card_container}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
