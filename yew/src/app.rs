use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{About, Blog, BlogPostPage, Home, Resume};
use crate::styles::components;

#[derive(Clone, Debug, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    BlogPost { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    console::log_1(&"App component rendering".into());

    html! {
        <BrowserRouter>
            <div class={components::MAIN_CONTAINER}>
                <nav class={components::NAV_CONTAINER}>
                    <div class={components::NAV_INNER}>
                        <a href="/" class="text-2xl font-bold text-gray-900 no-underline hover:text-primary-600 transition-colors duration-200">
                            {"Matthew Elders"}
                        </a>
                        <ul class="flex list-none gap-8">
                            <li>
                                <Link<Route> to={Route::Home} classes={components::NAV_LINK}>{"Home"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Resume} classes={components::NAV_LINK}>{"Resume"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Blog} classes={components::NAV_LINK}>{"Blog"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::About} classes={components::NAV_LINK}>{"About"}</Link<Route>>
                            </li>
                        </ul>
                    </div>
                </nav>

                <main class={components::CONTENT_CONTAINER}>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    console::log_1(&format!("Routing to: {:?}", routes).into());

    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::Blog => html! { <Blog /> },
        Route::BlogPost { slug } => html! { <BlogPostPage {slug} /> },
        Route::NotFound => html! {
            <div class="text-center py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">{"404 - Page Not Found"}</h1>
                <p class="text-gray-600 mb-8">{"The page you're looking for doesn't exist."}</p>
                <Link<Route> to={Route::Home} classes={components::BTN_PRIMARY}>{"Go Home"}</Link<Route>>
            </div>
        },
    }
}
