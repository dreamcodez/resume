# Blog Posts Directory

This directory contains all blog posts for the Yew/WebAssembly resume site. Each post is a Markdown file with YAML frontmatter for metadata.

## How to Add a New Blog Post

1. **Create a new `.md` file** in this directory
2. **Add YAML frontmatter** at the top of the file (see format below)
3. **Write your content** in Markdown format
4. **Save the file** - it will automatically appear in the blog

## Frontmatter Format

Every blog post must start with YAML frontmatter between `---` markers:

```yaml
---
title: "Your Post Title"
date: "2024-06-01"
slug: "your-post-slug"
tags: ["rust", "yew", "webassembly"]
summary: "A brief description of your post (shown in blog index)"
draft: false
author: "Your Name"
reading_time: 5
---
```

## Required Fields

- **`title`**: The post title (string)
- **`date`**: Publication date (YYYY-MM-DD format)
- **`slug`**: URL-friendly identifier (lowercase, hyphens, no spaces)
- **`tags`**: Array of tags for categorization
- **`summary`**: Brief description shown in blog index

## Optional Fields

- **`draft`**: Set to `true` to hide the post (default: `false`)
- **`author`**: Author name (default: "Matthew Elders")
- **`reading_time`**: Estimated reading time in minutes (auto-calculated if omitted)

## Example Blog Post

````markdown
---
title: "Building a Modern Web App with Rust and Yew"
date: "2024-06-01"
slug: "building-modern-web-app-rust-yew"
tags: ["rust", "yew", "webassembly", "tutorial"]
summary: "Learn how to build a high-performance web application using Rust and the Yew framework."
draft: false
author: "Matthew Elders"
reading_time: 8
---

# Building a Modern Web App with Rust and Yew

This is the introduction paragraph. You can use all standard Markdown formatting.

## Section Heading

Regular paragraph text goes here. You can include:

- **Bold text**
- _Italic text_
- `Code snippets`
- [Links](https://example.com)

### Code Blocks

```rust
fn main() {
    println!("Hello, Yew!");
}
```
````

## Lists

1. Numbered lists work too
2. Just like regular Markdown
3. With proper formatting

- Bullet points
- Are also supported
- With proper indentation

## Images

You can include images using standard Markdown:

![Alt text](/static/image.jpg)

## Conclusion

Wrap up your post with a conclusion paragraph.

```

## Tips for Writing Posts

1. **Use descriptive slugs**: Make them URL-friendly and descriptive
2. **Write good summaries**: Keep them under 160 characters for better SEO
3. **Use relevant tags**: Help readers find related content
4. **Test your markdown**: Preview the formatting before publishing
5. **Set draft status**: Use `draft: true` while writing, change to `false` when ready

## Draft Posts

Posts with `draft: true` in the frontmatter will not appear in the blog index or be accessible via direct URL. This is useful for:

- Posts you're still writing
- Posts you want to review before publishing
- Seasonal content that's not ready yet

## Reading Time

If you don't specify `reading_time`, it will be automatically calculated based on word count (approximately 200 words per minute).

## File Naming

Use descriptive filenames that match your slug:
- Good: `building-modern-web-app-rust-yew.md`
- Avoid: `post1.md`, `new-post.md`

## Need Help?

- Check existing posts for examples
- Refer to the main project README for technical details
- Use standard Markdown syntax for formatting
```
