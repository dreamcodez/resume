use crate::models::{BlogPost, BlogPostFrontmatter};
use pulldown_cmark::{html, Options, Parser};
use serde_yaml;
use std::fs;
use std::path::Path;

pub fn get_blog_posts() -> Vec<BlogPost> {
    let dir = Path::new("src/data/blog_posts");
    let mut posts = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    if let Ok(text) = fs::read_to_string(entry.path()) {
                        if let Some((frontmatter, content)) = split_frontmatter(&text) {
                            if let Ok(meta) =
                                serde_yaml::from_str::<BlogPostFrontmatter>(&frontmatter)
                            {
                                // Skip draft posts
                                if meta.draft.unwrap_or(false) {
                                    continue;
                                }

                                let html_content = markdown_to_html(&content);
                                let _reading_time = meta
                                    .reading_time
                                    .unwrap_or_else(|| calculate_reading_time(&content));

                                posts.push(BlogPost {
                                    slug: meta.slug,
                                    title: meta.title,
                                    date: meta.date,
                                    content: html_content,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn split_frontmatter(text: &str) -> Option<(String, String)> {
    let mut lines = text.lines();
    if lines.next()? != "---" {
        return None;
    }
    let mut frontmatter = String::new();
    for line in &mut lines {
        if line == "---" {
            let rest = lines.collect::<Vec<_>>().join("\n");
            return Some((frontmatter.trim().to_string(), rest.trim().to_string()));
        }
        frontmatter.push_str(line);
        frontmatter.push('\n');
    }
    None
}

fn markdown_to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn calculate_reading_time(content: &str) -> u32 {
    // Average reading speed: 200-250 words per minute
    // Using 225 words per minute as a reasonable average
    let word_count = content.split_whitespace().count();
    let minutes = (word_count as f64 / 225.0).ceil() as u32;
    minutes.max(1) // Minimum 1 minute reading time
}

pub fn get_blog_post_by_slug(slug: &str) -> Option<BlogPost> {
    get_blog_posts().into_iter().find(|post| post.slug == slug)
}
