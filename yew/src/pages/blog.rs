use crate::models::BlogPost;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub slug: String,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    // TODO: Load blog posts from API or static files
    let posts = vec![BlogPost {
        slug: "built-a-new-homepage".to_string(),
        title: "Built a New Homepage".to_string(),
        date: "2023-01-01".to_string(),
        content: "".to_string(),
    }];

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

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    // TODO: Load specific blog post content
    html! {
        <div class="blog-post-container">
            <h1>{"Blog Post: "}{&props.slug}</h1>
            <p>{"Content for blog post will be loaded here..."}</p>
        </div>
    }
}
