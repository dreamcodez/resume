use web_sys::console;
use yew::prelude::*;

use crate::components::router::{get_current_route_info, parse_route, Link, RouteInfo, Router};
use crate::pages::{About, Blog, BlogPostPage, Home, Resume};

#[function_component(App)]
pub fn app() -> Html {
    console::log_1(&"App component rendering".into());

    let on_route_change = Callback::from(|route_info: RouteInfo| {
        log::info!("Route changed to: {}", route_info.path);
    });

    html! {
        <Router on_route_change={on_route_change}>
            <div class="main-container">
                <nav class="nav-container">
                    <div class="nav-inner">
                        <Link to="/" class="text-2xl font-bold text-gray-900 no-underline hover:text-blue-600 transition-colors duration-200">
                            {"Matthew Elders"}
                        </Link>
                        <ul class="flex list-none gap-8">
                            <li>
                                <Link to="/" class="nav-link">{"Home"}</Link>
                            </li>
                            <li>
                                <Link to="/resume" class="nav-link">{"Resume"}</Link>
                            </li>
                            <li>
                                <Link to="/blog" class="nav-link">{"Blog"}</Link>
                            </li>
                            <li>
                                <Link to="/about" class="nav-link">{"About"}</Link>
                            </li>
                        </ul>
                    </div>
                </nav>

                <main class="content-container">
                    <RouteContent />
                </main>
            </div>
        </Router>
    }
}

#[function_component(RouteContent)]
fn route_content() -> Html {
    let route_info = get_current_route_info();

    match route_info.path.as_str() {
        "/" => html! { <Home /> },
        "/about" => html! { <About /> },
        "/resume" => html! { <Resume /> },
        "/blog" => html! { <Blog /> },
        path => {
            // Check for blog post routes
            if let Some(params) = parse_route(path, "/blog/:slug") {
                let slug = params.get("slug").unwrap().clone();
                html! { <BlogPostPage {slug} /> }
            } else {
                // 404 page
                html! {
                    <div class="page-container section-container text-center">
                        <div class="fade-in">
                            <h1 class="text-6xl text-red-600 mb-4">{"404"}</h1>
                            <p class="mb-6 text-gray-600">{"Page not found"}</p>
                            <Link to="/" class="btn btn-primary">
                                {"← Go back home"}
                            </Link>
                        </div>
                    </div>
                }
            }
        }
    }
}
