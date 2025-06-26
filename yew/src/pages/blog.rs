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
        <div class="blog-container">
            <h1>{"Recent posts"}</h1>
            <ul>
                {posts.into_iter().map(|post| {
                    html! {
                        <li>
                            <a href={format!("/blog/{}", post.slug)}>
                                {format!("{} [{}]", post.title, post.date)}
                            </a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

#[function_component(BlogPostPage)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    // TODO: Load specific blog post content
    html! {
        <div class="blog-post-container">
            <h1>{"Blog Post: "}{&props.slug}</h1>
            <p>{"Content for blog post will be loaded here..."}</p>
        </div>
    }
}
