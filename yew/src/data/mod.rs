pub mod blog;
pub mod jobs;
pub mod skills;

// Re-export all functions for easy access
pub use blog::{get_blog_post_by_slug, get_blog_posts};
pub use jobs::{get_job_by_company, get_jobs};
pub use skills::{get_skills, get_skills_by_level};
