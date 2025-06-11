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
        <div class={"card w-32 bg-base-200 border-1 items-center rounded-box shadow-sm "}>
            <img class={"bg-primary"} src={image_source} />
            <div class={"card-body"}>
                <h2 class={"card-title text-ls whitespace-nowrap h-1"}>
                    {job_name}
                </h2>
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let data = include_str!("data.yaml");
    let classes: HashMap<String, Vec<Job>> = serde_yaml::from_str(data).unwrap();

    let mut warrior_cards = vec![];

    for job in &classes["전사"] {
        let card = generate_job_card(&job.name, &job.src);
        warrior_cards.push(card);
    }

    let characters = html! {
        <div class={"grid grid-cols-5 gap-4"}>
            {for warrior_cards}
        </div>
    };

    html! {
        <div class={"flex justify-center"}>
            {characters}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
