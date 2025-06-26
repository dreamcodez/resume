use crate::models::{Job, Skill, SkillLevel};
use yew::prelude::*;

#[function_component(Resume)]
pub fn resume() -> Html {
    // TODO: Load data from API or static files
    let jobs = vec![
        Job {
            company: "Umee (Now UX Chain)".to_string(),
            company_description: "UX Chain, formerly UMEE, is a layer-one blockchain within the Cosmos ecosystem, dedicated to enhancing user experience and fostering innovation in decentralised finance (DeFi).".to_string(),
            company_website: "https://www.ux.xyz".to_string(),
            job_title: "Head Infrastructure Engineer".to_string(),
            start: "2021-12".to_string(),
            end: Some("2023-03".to_string()),
            markdown: "Automated deployment of several blockchain test networks...".to_string(),
        },
    ];

    let skills = vec![
        Skill {
            name: "Javascript / NodeJS".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Rust".to_string(),
            level: SkillLevel::Fluent,
        },
    ];

    html! {
        <div class="resume-container">
            <p>
                {"This web-based resume is a work in progress. For the 'stable' version of my resume:"}
            </p>

            <ul>
                <li><a href="https://github.com/dreamcodez/resume/blob/main/README.md">{"Markdown (GitHub) version"}</a></li>
                <li><a href="https://github.com/dreamcodez/resume/raw/main/README.pdf">{"PDF Version"}</a></li>
            </ul>

            <h1>{"Experience"}</h1>
            <ul class="experience">
                {jobs.into_iter().map(|job| {
                    html! {
                        <li class="job-item">
                            <h2>
                                <a class="company" href={job.company_website.clone()} target="_blank">
                                    {job.company.clone()} {" 🔗"}
                                </a>
                            </h2>
                            <h3>{job.job_title}</h3>
                            <h4>{format!("{} to {}", job.start, job.end.as_deref().unwrap_or("Present"))}</h4>
                            <p class="company-description">{job.company_description}</p>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>

            <h1>{"Skills"}</h1>

            <h2>{"Expert"}</h2>
            <ul>
                {skills.iter().filter(|s| matches!(s.level, SkillLevel::Expert)).map(|skill| {
                    html! { <li>{&skill.name}</li> }
                }).collect::<Html>()}
            </ul>

            <h2>{"Fluent"}</h2>
            <ul>
                {skills.iter().filter(|s| matches!(s.level, SkillLevel::Fluent)).map(|skill| {
                    html! { <li>{&skill.name}</li> }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}
