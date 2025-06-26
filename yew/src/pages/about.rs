use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="about-container">
            <h1>{"About this site"}</h1>

            <h2>{"The Stack"}</h2>
            <ul>
                <li>{"This site is built in "}<a href="https://yew.rs">{"Yew + Rust"}</a></li>
                <li>{"WebAssembly for client-side rendering"}</li>
                <li>{"Trunk for building and bundling"}</li>
                <li>{"The code is available on github "}<a href="https://github.com/dreamcodez/resume">{"here"}</a></li>
            </ul>
        </div>
    }
}
