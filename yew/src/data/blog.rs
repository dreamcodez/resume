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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_frontmatter() {
        let test_content = r#"---
title: "Test Post"
date: "2024-01-01"
slug: "test-post"
---
# Test Content
This is the content."#;

        let result = split_frontmatter(test_content);
        assert!(result.is_some());

        let (frontmatter, content) = result.unwrap();
        assert!(frontmatter.contains("title: \"Test Post\""));
        assert!(frontmatter.contains("date: \"2024-01-01\""));
        assert!(frontmatter.contains("slug: \"test-post\""));
        assert!(content.contains("# Test Content"));
        assert!(content.contains("This is the content."));
    }

    #[test]
    fn test_split_frontmatter_no_frontmatter() {
        let test_content = "# Test Content\nThis is the content.";
        let result = split_frontmatter(test_content);
        assert!(result.is_none());
    }

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Test Heading\n\nThis is **bold** text.";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<h1>Test Heading</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_calculate_reading_time() {
        // Test with 225 words (should be 1 minute)
        let content = "word ".repeat(225);
        assert_eq!(calculate_reading_time(&content), 1);

        // Test with 450 words (should be 2 minutes)
        let content = "word ".repeat(450);
        assert_eq!(calculate_reading_time(&content), 2);

        // Test with empty content (should be 1 minute minimum)
        assert_eq!(calculate_reading_time(""), 1);
    }

    #[test]
    fn test_frontmatter_deserialization() {
        let yaml = r#"
title: "Test Post"
date: "2024-01-01"
slug: "test-post"
tags: ["rust", "webassembly"]
summary: "A test post"
draft: false
author: "Matthew Elder <matt@jupitersoft.net>"
reading_time: 5
"#;

        let result: Result<BlogPostFrontmatter, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());

        let frontmatter = result.unwrap();
        assert_eq!(frontmatter.title, "Test Post");
        assert_eq!(frontmatter.date, "2024-01-01");
        assert_eq!(frontmatter.slug, "test-post");
        assert_eq!(
            frontmatter.tags,
            Some(vec!["rust".to_string(), "webassembly".to_string()])
        );
        assert_eq!(frontmatter.summary, Some("A test post".to_string()));
        assert_eq!(frontmatter.draft, Some(false));
        assert_eq!(
            frontmatter.author,
            Some("Matthew Elder <matt@jupitersoft.net>".to_string())
        );
        assert_eq!(frontmatter.reading_time, Some(5));
    }

    #[test]
    fn test_frontmatter_optional_fields() {
        let yaml = r#"
title: "Test Post"
date: "2024-01-01"
slug: "test-post"
"#;

        let result: Result<BlogPostFrontmatter, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());

        let frontmatter = result.unwrap();
        assert_eq!(frontmatter.title, "Test Post");
        assert_eq!(frontmatter.date, "2024-01-01");
        assert_eq!(frontmatter.slug, "test-post");
        assert_eq!(frontmatter.tags, None);
        assert_eq!(frontmatter.summary, None);
        assert_eq!(frontmatter.draft, None);
        assert_eq!(frontmatter.author, None);
        assert_eq!(frontmatter.reading_time, None);
    }

    #[test]
    fn test_get_blog_posts_returns_posts() {
        let posts = get_blog_posts();
        // Should have at least the two blog posts we created
        assert!(posts.len() >= 2);

        // Posts should be sorted by date descending
        if posts.len() >= 2 {
            assert!(posts[0].date >= posts[1].date);
        }

        // All posts should have required fields
        for post in posts {
            assert!(!post.title.is_empty());
            assert!(!post.slug.is_empty());
            assert!(!post.date.is_empty());
            assert!(!post.content.is_empty());
        }
    }

    #[test]
    fn test_get_blog_post_by_slug() {
        let post = get_blog_post_by_slug("built-a-new-homepage");
        assert!(post.is_some());

        let post = post.unwrap();
        assert_eq!(post.slug, "built-a-new-homepage");
        assert!(post.title.contains("Rust and WebAssembly"));

        // Non-existent slug should return None
        let post = get_blog_post_by_slug("non-existent-post");
        assert!(post.is_none());
    }
}
