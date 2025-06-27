use web_sys::console;
use yew::prelude::*;
// use yew_router::prelude::*;  // Temporarily commented out

use crate::pages::{About, Blog, BlogPostPage, Home, Resume};

// #[derive(Clone, Debug, Routable, PartialEq)]
// enum Route {
//     #[at("/")]
//     Home,
//     #[at("/about")]
//     About,
//     #[at("/resume")]
//     Resume,
//     #[at("/blog")]
//     Blog,
//     #[at("/blog/:slug")]
//     BlogPost { slug: String },
//     #[not_found]
//     #[at("/404")]
//     NotFound,
// }

#[function_component(App)]
pub fn app() -> Html {
    console::log_1(&"App component rendering".into());

    html! {
        // <BrowserRouter>  // Temporarily commented out
            <div class="main-container">
                <nav class="nav-container">
                    <div class="nav-inner">
                        <a href="/" class="text-2xl font-bold text-gray-900 no-underline hover:text-blue-600 transition-colors duration-200">
                            {"Matthew Elders"}
                        </a>
                        <ul class="flex list-none gap-8">
                            <li>
                                // <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>  // Temporarily commented out
                                <a href="/" class="nav-link">{"Home"}</a>
                            </li>
                            <li>
                                // <Link<Route> to={Route::Resume} classes="nav-link">{"Resume"}</Link<Route>>  // Temporarily commented out
                                <a href="/resume" class="nav-link">{"Resume"}</a>
                            </li>
                            <li>
                                // <Link<Route> to={Route::Blog} classes="nav-link">{"Blog"}</Link<Route>>  // Temporarily commented out
                                <a href="/blog" class="nav-link">{"Blog"}</a>
                            </li>
                            <li>
                                // <Link<Route> to={Route::About} classes="nav-link">{"About"}</Link<Route>>  // Temporarily commented out
                                <a href="/about" class="nav-link">{"About"}</a>
                            </li>
                        </ul>
                    </div>
                </nav>

                <main class="content-container">
                    // <Switch<Route> render={switch} />  // Temporarily commented out
                    <Home />
                </main>
            </div>
        // </BrowserRouter>  // Temporarily commented out
    }
}

// fn switch(routes: Route) -> Html {  // Temporarily commented out
//     match routes {
//         Route::Home => html! { <Home /> },
//         Route::About => html! { <About /> },
//         Route::Resume => html! { <Resume /> },
//         Route::Blog => html! { <Blog /> },
//         Route::BlogPost { slug } => html! { <BlogPostPage {slug} /> },
//         Route::NotFound => html! {
//             <div class="page-container section-container text-center">
//                 <div class="fade-in">
//                     <h1 class="text-6xl text-red-600 mb-4">{"404"}</h1>
//                     <p class="mb-6 text-gray-600">{"Page not found"}</p>
//                     <a href="/" class="btn btn-primary">
//                         {"← Go back home"}
//                     </a>
//                 </div>
//             </div>
//         },
//     }
// }
