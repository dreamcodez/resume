use crate::components::InteractivePuzzle;
use crate::styles::home;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let on_puzzle_solved = Callback::from(|_: ()| {
        // Could add analytics or other side effects here
        web_sys::console::log_1(&"Puzzle solved!".into());
    });

    html! {
        <div class={home::CONTAINER}>
            <div class="mb-8">
                <h2 class={format!("{} {} {}",
                    home::HERO_HEADING,
                    "animate-pulse",
                    "bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent"
                )}>
                    {"With a triumphant beep boop, his status changed:"}
                </h2>
                <div class={format!("{} {} {} {}",
                    home::HERO_CODE,
                    "bg-gray-900 text-green-400 p-4 rounded-lg",
                    "font-mono text-lg",
                    "animate-pulse border-l-4 border-green-400"
                )}>
                    <span class="animate-bounce">{"⚡"}</span> {"let isHired = true;"}
                </div>
            </div>

            <InteractivePuzzle on_solved={Some(on_puzzle_solved)} />

            <h3 class={format!("{} {} {}",
                home::NAME_HEADING,
                "bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent",
                "hover:scale-105 transition-transform duration-200"
            )}>
                <span class="text-blue-600 animate-pulse">{"M"}</span>{"atthew Elder"}
            </h3>

            <div class="mb-6">
                <em class={format!("{} {} {}",
                    home::NAME_EM,
                    "bg-blue-100 text-blue-800 px-3 py-1 rounded-full",
                    "inline-block"
                )}>
                    {"🚀 Founder, Technical Architect, Startup Partner"}
                </em>
            </div>

            <div class="space-y-6">
                <div class={format!("{} {} {}",
                    "p-6 rounded-xl border-l-4 border-blue-500",
                    "bg-gradient-to-r from-blue-50 to-white",
                    "hover:shadow-lg transition-shadow duration-200"
                )}>
                    <p class={home::PARAGRAPH}>
                        {"Matthew Elder is a seasoned software architect and startup technologist with 20+ years of experience designing scalable systems, building full-stack applications, and leading high-performance engineering teams. He has helped launch and scale dozens of products across industries—from fintech and blockchain to media, communications, and SaaS platforms."}
                    </p>
                </div>

                <div class={format!("{} {} {}",
                    "p-6 rounded-xl border-l-4 border-green-500",
                    "bg-gradient-to-r from-green-50 to-white",
                    "hover:shadow-lg transition-shadow duration-200"
                )}>
                    <p class={home::PARAGRAPH}>
                        {"Matthew's superpower lies in translating ambitious startup ideas into lean, functional, and future-proof technology. With deep expertise in systems architecture, cloud infrastructure, DevOps, and iterative product development, he partners closely with founders to make smart technical decisions from day one. Whether you're launching an MVP, replatforming a legacy product, or scaling a critical system under load, Matthew brings tactical clarity and senior-level execution to every engagement."}
                    </p>
                </div>
            </div>

            <div class="mt-8 p-6 bg-gray-50 rounded-xl">
                <h4 class="text-lg font-semibold mb-4 text-gray-800">{"🛠️ Built with Modern Tech"}</h4>
                <div class="flex flex-wrap gap-2">
                    <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm font-medium hover:bg-blue-200 transition-colors">{"Rust"}</span>
                    <span class="bg-orange-100 text-orange-800 px-3 py-1 rounded-full text-sm font-medium hover:bg-orange-200 transition-colors">{"WebAssembly"}</span>
                    <span class="bg-cyan-100 text-cyan-800 px-3 py-1 rounded-full text-sm font-medium hover:bg-cyan-200 transition-colors">{"Yew"}</span>
                    <span class="bg-purple-100 text-purple-800 px-3 py-1 rounded-full text-sm font-medium hover:bg-purple-200 transition-colors">{"TailwindCSS"}</span>
                </div>
            </div>
        </div>
    }
}
