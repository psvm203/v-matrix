use serde::Deserialize;
use std::collections::HashMap;
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
        let data = [
            ("전사", "btn-secondary"),
            ("마법사", "btn-info"),
            ("궁수", "btn-accent"),
            ("도적", "btn-primary"),
            ("해적", "btn-neutral"),
        ];

        let mut buttons = vec![];

        for &(class_name, style) in &data {
            let styles = format!("btn {style}");
            let button: Html = html! {
                <button class={styles}>
                    {class_name}
                </button>
            };
            buttons.push(button);
        }

        html! {
            <div class={"flex justify-center"}>
                <div class={"grid grid-cols-5 gap-6"}>
                    {for buttons}
                </div>
            </div>
        }
    };

    let data = include_str!("data.yaml");
    let classes: HashMap<String, Vec<Job>> = serde_yaml::from_str(data).unwrap();
    let mut warrior_cards = vec![];

    for job in &classes["전사"] {
        let card = generate_job_card(&job.name, &job.src);
        warrior_cards.push(card);
    }

    let card_container: Html = html! {
        <div class={"flex justify-center"}>
            <div class={"grid grid-cols-5 gap-4"}>
                {for warrior_cards}
            </div>
        </div>
    };

    html! {
        <div class={"mt-4 grid grid-cols-1 gap-4"}>
            {class_button_container}
            {card_container}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
