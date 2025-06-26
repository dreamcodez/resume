use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{About, Blog, BlogPost, Home, Resume};

#[derive(Clone, Routable, PartialEq)]
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
    html! {
        <BrowserRouter>
            <div class="app">
                <nav class="navbar">
                    <div class="nav-container">
                        <a href="/" class="nav-logo">{"Matthew Elders"}</a>
                        <ul class="nav-menu">
                            <li class="nav-item">
                                <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Resume} classes="nav-link">{"Resume"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Blog} classes="nav-link">{"Blog"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::About} classes="nav-link">{"About"}</Link<Route>>
                            </li>
                        </ul>
                    </div>
                </nav>

                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::Blog => html! { <Blog /> },
        Route::BlogPost { slug } => html! { <BlogPost {slug} /> },
        Route::NotFound => html! {
            <div class="not-found">
                <h1>{"404 - Page Not Found"}</h1>
                <p>{"The page you're looking for doesn't exist."}</p>
                <Link<Route> to={Route::Home}>{"Go Home"}</Link<Route>>
            </div>
        },
    }
}
