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
    Amateur,
    Basics,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BlogPostFrontmatter {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub draft: Option<bool>,
    pub author: Option<String>,
    pub reading_time: Option<u32>, // in minutes
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Resume {
    pub jobs: Vec<Job>,
    pub skills: Vec<Skill>,
}

// Additional models for enhanced features
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field: String,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub date_earned: String,
    pub expiry_date: Option<String>,
    pub credential_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact {
    pub email: String,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

// Performance metrics for technical showcase
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PerformanceMetrics {
    pub initial_load_time: u32,   // milliseconds
    pub time_to_interactive: u32, // milliseconds
    pub bundle_size: u32,         // bytes
    pub lighthouse_score: u8,     // 0-100
    pub core_web_vitals: CoreWebVitals,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CoreWebVitals {
    pub lcp: u32, // Largest Contentful Paint (ms)
    pub fid: u32, // First Input Delay (ms)
    pub cls: f32, // Cumulative Layout Shift
}

// Theme and UI state
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppState {
    pub theme: Theme,
    pub current_route: String,
    pub performance_metrics: Option<PerformanceMetrics>,
}
