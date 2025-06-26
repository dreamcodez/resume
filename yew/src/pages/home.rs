use crate::styles::components;
use crate::styles::home;
use crate::styles::{animations, colors, effects, layout, typography};
use web_sys::console;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    console::log_1(&"Home component rendering".into());

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

            <div class="relative w-full max-w-4xl mx-auto aspect-[16/5] rounded-2xl shadow-lg group cursor-pointer bg-gradient-to-br from-blue-50 to-purple-50 overflow-hidden mb-8">
                <img
                    alt="Matthew Elder & Team - Interactive Puzzle"
                    class="absolute inset-0 w-full h-full object-cover transition-all duration-300 group-hover:brightness-75 group-hover:contrast-125 filter grayscale group-hover:grayscale-0"
                    src="/static/sophisticated-macman.jpg"
                />
                <div class="absolute inset-0 pointer-events-none group-hover:pointer-events-auto bg-black bg-opacity-0 group-hover:bg-opacity-20 transition-all duration-300 flex items-center justify-center opacity-0 group-hover:opacity-100">
                    <div class="absolute top-4 left-4 bg-green-900 text-green-400 p-2 rounded text-xs font-mono animate-bounce">
                        {"fn solve_puzzle() -> bool"}
                    </div>
                    <div class="absolute top-12 right-8 bg-blue-900 text-blue-400 p-2 rounded text-xs font-mono animate-pulse">
                        {"let solution = true;"}
                    </div>
                    <div class="absolute bottom-8 left-8 bg-purple-900 text-purple-400 p-2 rounded text-xs font-mono animate-ping">
                        {"match result {"}
                    </div>
                    <div class="absolute top-1/2 left-1/4 transform -translate-x-1/2 -translate-y-1/2">
                        <div class="bg-orange-500 text-white p-3 rounded-full text-2xl animate-spin">
                            {"🦀"}
                        </div>
                    </div>
                    <div class="absolute top-1/3 right-1/4 transform translate-x-1/2 -translate-y-1/2">
                        <div class="bg-cyan-500 text-white p-3 rounded-full text-2xl animate-bounce">
                            {"⚡"}
                        </div>
                    </div>
                    <div class="absolute bottom-1/3 right-1/3 transform translate-x-1/2 translate-y-1/2">
                        <div class="bg-green-500 text-white p-3 rounded-full text-2xl animate-pulse">
                            {"🔧"}
                        </div>
                    </div>
                    <div class="absolute bottom-4 right-4 bg-white bg-opacity-90 p-3 rounded-lg text-sm text-gray-800">
                        <div class="font-bold mb-1">{"🧩 Interactive Puzzle"}</div>
                        <div>{"Hover to reveal hidden code!"}</div>
                    </div>
                </div>
                <div class="absolute top-2 right-2 bg-yellow-400 text-yellow-900 px-2 py-1 rounded-full text-xs font-bold opacity-0 group-hover:opacity-100 transition-opacity animate-pulse">
                    {"Click to Solve!"}
                </div>
            </div>

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
