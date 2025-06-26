use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home-container">
            <h2 class="home-subtitle">
                {"With a triumphant beep boop, his status changed:"}
            </h2>
            <code class="status-code">{"let isHired = true;"}</code>

            <div class="home-content">
                <img
                    alt="Matthew Elder & Team"
                    class="home-image"
                    src="/sophisticated-macman.jpg"
                />
                <div class="home-text">
                    <h3 class="home-title">
                        <span class="home-initial">{"M"}</span>{"atthew Elder"}
                    </h3>
                    <em class="home-subtitle">{"Founder, Technical Architect, Startup Partner"}</em>
                    <p>
                        {"Matthew Elder is a seasoned software architect and startup technologist with
                        20+ years of experience designing scalable systems, building full-stack 
                        applications, and leading high-performance engineering teams. He has helped 
                        launch and scale dozens of products across industries—from fintech and 
                        blockchain to media, communications, and SaaS platforms."}
                    </p>
                    <p>
                        {"Matthew's superpower lies in translating ambitious startup ideas into lean,
                        functional, and future-proof technology. With deep expertise in systems 
                        architecture, cloud infrastructure, DevOps, and iterative product 
                        development, he partners closely with founders to make smart technical 
                        decisions from day one. Whether you're launching an MVP, replatforming a 
                        legacy product, or scaling a critical system under load, Matthew brings 
                        tactical clarity and senior-level execution to every engagement."}
                    </p>
                </div>
            </div>
        </div>
    }
}
