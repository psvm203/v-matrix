use yew::prelude::*;

fn generate_job(job_name: &str, image_name: &str) -> Html {
    let image_source = format!("assets/{image_name}.png");

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
    let warriors_data = [
        ("히어로", "hero"),
        ("팔라딘", "paladin"),
        ("다크나이트", "dark_knight"),
        ("소울마스터", "soul_master"),
        ("미하일", "mihile"),
        ("블래스터", "blaster"),
        ("데몬 슬레이어", "demon_slayer"),
        ("데몬 어벤져", "demon_avenger"),
        ("아란", "aran"),
        ("카이저", "kaiser"),
        ("아델", "adele"),
        ("제로", "zero"),
        ("렌", "len"),
    ];

    let mut warriors = vec![];

    for &(job_name, image_name) in &warriors_data {
        let job = generate_job(job_name, image_name);
        warriors.push(job);
    }

    let characters = html! {
        <div class={"grid grid-cols-5 gap-4"}>
            {for warriors}
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
