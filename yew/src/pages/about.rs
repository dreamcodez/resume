use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="page-container section-container">
            <div class="fade-in">
                <h1>{"About this site"}</h1>

                <div class="about-section">
                    <h2>{"The Stack"}</h2>
                    <ul class="about-list">
                        <li>{"This site is built in "}<a href="https://yew.rs">{"Yew + Rust"}</a></li>
                        <li>{"WebAssembly for client-side rendering"}</li>
                        <li>{"Trunk for building and bundling"}</li>
                        <li>{"Tailwind CSS for styling"}</li>
                        <li>{"The code is available on github "}<a href="https://github.com/dreamcodez/resume">{"here"}</a></li>
                    </ul>
                </div>

                <div class="about-section">
                    <h2>{"Features"}</h2>
                    <ul class="about-list">
                        <li>{"Responsive design that works on all devices"}</li>
                        <li>{"Fast loading with WebAssembly"}</li>
                        <li>{"Modern UI with smooth animations"}</li>
                        <li>{"Accessible design following WCAG guidelines"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
