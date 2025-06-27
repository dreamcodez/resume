use crate::data::get_blog_posts;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub slug: String,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts = get_blog_posts();

    html! {
        <div class="page-container section-container">
            <div class="fade-in">
                <h1>{"Recent posts"}</h1>

                <div class="content-spacing">
                    {posts.clone().into_iter().enumerate().map(|(index, post)| {
                        html! {
                            <div class={format!("blog-item slide-up")} style={format!("animation-delay: {}ms", index * 100)}>
                                <a href={format!("/blog/{}", post.slug)} class="blog-link">
                                    {post.title}
                                </a>
                                <div class="blog-meta">
                                    {format!("Published on {}", post.date)}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>

                {if posts.is_empty() {
                    html! {
                        <div class="text-center py-12">
                            <p class="text-gray-500">{"No blog posts available yet."}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

#[function_component(BlogPostPage)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    // TODO: Load specific blog post content
    html! {
        <div class="page-container section-container">
            <div class="fade-in">
                <h1>{"Blog Post: "}{&props.slug}</h1>
                <div class="card">
                    <p>{"Content for blog post will be loaded here..."}</p>
                    <div class="mt-6">
                        <a href="/blog" class="btn btn-secondary">
                            {"← Back to Blog"}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
