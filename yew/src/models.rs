use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Job {
    pub company: String,
    pub company_description: String,
    pub company_website: String,
    pub job_title: String,
    pub start: String,
    pub end: Option<String>,
    pub markdown: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: SkillLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SkillLevel {
    Expert,
    Fluent,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Resume {
    pub jobs: Vec<Job>,
    pub skills: Vec<Skill>,
}
