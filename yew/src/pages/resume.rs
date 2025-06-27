use crate::data::{get_education, get_jobs, get_skills};
use crate::models::SkillLevel;
use yew::prelude::*;

#[function_component(Resume)]
pub fn resume() -> Html {
    let jobs = get_jobs();
    let skills = get_skills();
    let education = get_education();

    html! {
        <div class="page-container section-container">
            <div class="fade-in">
                <p class="mb-6">
                    {"This web-based resume is a work in progress. For the 'stable' version of my resume:"}
                </p>

                <ul class="mb-8 pl-8">
                    <li class="mb-2"><a href="https://github.com/dreamcodez/resume/blob/main/README.md">{"Markdown (GitHub) version"}</a></li>
                    <li class="mb-2"><a href="https://github.com/dreamcodez/resume/raw/main/README.pdf">{"PDF Version"}</a></li>
                </ul>

                <h1>{"Experience"}</h1>
                <div class="content-spacing">
                    {jobs.into_iter().enumerate().map(|(index, job)| {
                        html! {
                            <div class={format!("job-card slide-up")} style={format!("animation-delay: {}ms", index * 100)}>
                                <h2>
                                    <a class="company-link" href={job.company_website.clone()} target="_blank">
                                        {job.company.clone()} {" 🔗"}
                                    </a>
                                </h2>
                                <h3>{job.job_title}</h3>
                                <h4>{format!("{} to {}", job.start, job.end.as_deref().unwrap_or("Present"))}</h4>
                                <p class="company-description">{job.company_description}</p>

                                {if !job.markdown.is_empty() {
                                    html! {
                                        <div class="mt-4 prose prose-sm max-w-none">
                                            <div class="whitespace-pre-line text-gray-700 leading-relaxed">
                                                {job.markdown.split('\n').map(|line| {
                                                    if line.trim().starts_with("* **") {
                                                        html! {
                                                            <div class="ml-4 mb-2">
                                                                <span class="font-semibold text-gray-900">
                                                                    {line.trim_start_matches("* **").split("**").next().unwrap_or("")}
                                                                </span>
                                                                <span class="text-gray-700">
                                                                    {line.trim_start_matches("* **").split("**").nth(1).unwrap_or("")}
                                                                </span>
                                                            </div>
                                                        }
                                                    } else if line.trim().starts_with("* ") {
                                                        html! {
                                                            <div class="ml-4 mb-2 text-gray-700">
                                                                {line.trim_start_matches("* ")}
                                                            </div>
                                                        }
                                                    } else if !line.trim().is_empty() {
                                                        html! {
                                                            <div class="mb-2 text-gray-700">
                                                                {line}
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }).collect::<Html>()}
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    }).collect::<Html>()}
                </div>

                <h1>{"Skills"}</h1>
                <div class="skills-section">
                    <div class="skills-section">
                        <h2>{"Expert"}</h2>
                        <ul class="skills-grid">
                            {skills.iter().filter(|s| matches!(s.level, SkillLevel::Expert)).map(|skill| {
                                html! { <li class="skill-badge-expert">{&skill.name}</li> }
                            }).collect::<Html>()}
                        </ul>
                    </div>

                    <div class="skills-section">
                        <h2>{"Fluent"}</h2>
                        <ul class="skills-grid">
                            {skills.iter().filter(|s| matches!(s.level, SkillLevel::Fluent)).map(|skill| {
                                html! { <li class="skill-badge-fluent">{&skill.name}</li> }
                            }).collect::<Html>()}
                        </ul>
                    </div>

                    <div class="skills-section">
                        <h2>{"Amateur"}</h2>
                        <ul class="skills-grid">
                            {skills.iter().filter(|s| matches!(s.level, SkillLevel::Amateur)).map(|skill| {
                                html! { <li class="skill-badge">{&skill.name}</li> }
                            }).collect::<Html>()}
                        </ul>
                    </div>

                    <div class="skills-section">
                        <h2>{"Basics"}</h2>
                        <ul class="skills-grid">
                            {skills.iter().filter(|s| matches!(s.level, SkillLevel::Basics)).map(|skill| {
                                html! { <li class="skill-badge">{&skill.name}</li> }
                            }).collect::<Html>()}
                        </ul>
                    </div>
                </div>

                <h1>{"Education"}</h1>
                <div class="content-spacing">
                    {education.into_iter().enumerate().map(|(index, edu)| {
                        html! {
                            <div class={format!("job-card slide-up")} style={format!("animation-delay: {}ms", index * 100)}>
                                <h2>{edu.institution}</h2>
                                <h3>{format!("{} - {}", edu.degree, edu.field)}</h3>
                                <h4>{format!("{} to {}", edu.start_year, edu.end_year.as_ref().unwrap_or(&0))}</h4>
                                {if !edu.description.is_empty() {
                                    html! {
                                        <p class="company-description">{edu.description}</p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}
