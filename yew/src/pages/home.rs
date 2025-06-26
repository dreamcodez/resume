use crate::styles::components;
use crate::styles::{animations, colors, effects, layout, typography};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={layout::CONTAINER}>
            <div class="max-w-4xl mx-auto">
                // Hero Section
                <div class="text-center mb-16">
                    <h1 class={format!("{} {} {} {}",
                        typography::TEXT_4XL,
                        typography::FONT_BOLD,
                        colors::TEXT_GRAY_900,
                        animations::ANIMATE_FADE_IN
                    )}>
                        {"Matthew Elders"}
                    </h1>
                    <p class={format!("{} {} {} {} {}",
                        typography::TEXT_XL,
                        typography::FONT_MEDIUM,
                        colors::TEXT_PRIMARY_600,
                        "mt-4 mb-8",
                        animations::ANIMATE_SLIDE_UP
                    )}>
                        {"Technical Leader & Systems Architect"}
                    </p>
                    <div class={format!("{} {} {}",
                        "bg-gray-50 rounded-2xl p-6 mb-8",
                        effects::SHADOW_SM,
                        animations::ANIMATE_SCALE_IN
                    )}>
                        <code class={format!("{} {} {}",
                            typography::FONT_MONO,
                            typography::TEXT_LG,
                            colors::TEXT_PRIMARY_700
                        )}>
                            {"let technicalLeadership = true;"}
                        </code>
                    </div>
                </div>

                // Main Content
                <div class={format!("{} {}", layout::FLEX, "gap-12 items-start")}>
                    // Image Section
                    <div class="w-1/2">
                        <img
                            alt="Matthew Elders - Technical Leader"
                            class={format!("{} {} {}",
                                "w-full rounded-2xl",
                                effects::SHADOW_LG,
                                effects::TRANSITION_TRANSFORM
                            )}
                            src="/sophisticated-macman.jpg"
                        />
                    </div>

                    // Content Section
                    <div class="w-1/2 space-y-6">
                        <div>
                            <h2 class={format!("{} {} {} {}",
                                typography::TEXT_2XL,
                                typography::FONT_BOLD,
                                colors::TEXT_GRAY_900,
                                "mb-4"
                            )}>
                                {"20+ Years of Technical Excellence"}
                            </h2>
                            <p class={format!("{} {} {}",
                                typography::TEXT_BASE,
                                colors::TEXT_GRAY_700,
                                typography::LEADING_RELAXED
                            )}>
                                {"Matthew Elders is a seasoned technical leader and systems architect with over two decades of experience designing scalable systems, building full-stack applications, and leading high-performance engineering teams. He has helped launch and scale dozens of products across industries—from fintech and blockchain to media, communications, and SaaS platforms."}
                            </p>
                        </div>

                        <div>
                            <h3 class={format!("{} {} {} {}",
                                typography::TEXT_XL,
                                typography::FONT_SEMIBOLD,
                                colors::TEXT_GRAY_900,
                                "mb-3"
                            )}>
                                {"Technical Superpowers"}
                            </h3>
                            <ul class="space-y-2">
                                <li class={format!("{} {} {}",
                                    "flex items-center",
                                    colors::TEXT_GRAY_700,
                                    typography::TEXT_BASE
                                )}>
                                    <span class="text-success-500 mr-2">{"✓"}</span>
                                    {"Systems Architecture & Scalability"}
                                </li>
                                <li class={format!("{} {} {}",
                                    "flex items-center",
                                    colors::TEXT_GRAY_700,
                                    typography::TEXT_BASE
                                )}>
                                    <span class="text-success-500 mr-2">{"✓"}</span>
                                    {"Cloud Infrastructure & DevOps"}
                                </li>
                                <li class={format!("{} {} {}",
                                    "flex items-center",
                                    colors::TEXT_GRAY_700,
                                    typography::TEXT_BASE
                                )}>
                                    <span class="text-success-500 mr-2">{"✓"}</span>
                                    {"Blockchain & Distributed Systems"}
                                </li>
                                <li class={format!("{} {} {}",
                                    "flex items-center",
                                    colors::TEXT_GRAY_700,
                                    typography::TEXT_BASE
                                )}>
                                    <span class="text-success-500 mr-2">{"✓"}</span>
                                    {"Technical Leadership & Team Building"}
                                </li>
                            </ul>
                        </div>

                        <div>
                            <h3 class={format!("{} {} {} {}",
                                typography::TEXT_XL,
                                typography::FONT_SEMIBOLD,
                                colors::TEXT_GRAY_900,
                                "mb-3"
                            )}>
                                {"Startup Technology Partner"}
                            </h3>
                            <p class={format!("{} {} {}",
                                typography::TEXT_BASE,
                                colors::TEXT_GRAY_700,
                                typography::LEADING_RELAXED
                            )}>
                                {"Matthew's expertise lies in translating ambitious startup ideas into lean, functional, and future-proof technology. With deep knowledge in modern web technologies, cloud infrastructure, and iterative product development, he partners closely with founders to make smart technical decisions from day one."}
                            </p>
                        </div>

                        // Call to Action
                        <div class="pt-6">
                            <a
                                href="/resume"
                                class={format!("{} {} {}",
                                    components::BTN_PRIMARY,
                                    "mr-4",
                                    effects::TRANSITION
                                )}
                            >
                                {"View Full Resume"}
                            </a>
                            <a
                                href="/about"
                                class={format!("{} {}",
                                    components::BTN_SECONDARY,
                                    effects::TRANSITION
                                )}
                            >
                                {"Learn More"}
                            </a>
                        </div>
                    </div>
                </div>

                // Technical Showcase Section
                <div class="mt-16 pt-12 border-t border-gray-200">
                    <h2 class={format!("{} {} {} {}",
                        typography::TEXT_2XL,
                        typography::FONT_BOLD,
                        colors::TEXT_GRAY_900,
                        "text-center mb-8"
                    )}>
                        {"Built with Modern Technologies"}
                    </h2>
                    <div class={format!("{} {} {}",
                        layout::GRID,
                        layout::GRID_COLS_3,
                        layout::GAP_6
                    )}>
                        <div class={components::CARD}>
                            <h3 class={format!("{} {} {} {}",
                                typography::TEXT_LG,
                                typography::FONT_SEMIBOLD,
                                colors::TEXT_GRAY_900,
                                "mb-2"
                            )}>
                                {"Rust & WebAssembly"}
                            </h3>
                            <p class={format!("{} {}",
                                colors::TEXT_GRAY_600,
                                typography::TEXT_SM
                            )}>
                                {"Type-safe, high-performance frontend with near-native speed"}
                            </p>
                        </div>
                        <div class={components::CARD}>
                            <h3 class={format!("{} {} {} {}",
                                typography::TEXT_LG,
                                typography::FONT_SEMIBOLD,
                                colors::TEXT_GRAY_900,
                                "mb-2"
                            )}>
                                {"TailwindCSS"}
                            </h3>
                            <p class={format!("{} {}",
                                colors::TEXT_GRAY_600,
                                typography::TEXT_SM
                            )}>
                                {"Utility-first styling with type-safe Rust integration"}
                            </p>
                        </div>
                        <div class={components::CARD}>
                            <h3 class={format!("{} {} {} {}",
                                typography::TEXT_LG,
                                typography::FONT_SEMIBOLD,
                                colors::TEXT_GRAY_900,
                                "mb-2"
                            )}>
                                {"Performance Optimized"}
                            </h3>
                            <p class={format!("{} {}",
                                colors::TEXT_GRAY_600,
                                typography::TEXT_SM
                            )}>
                                {"Sub-100ms load times with 100/100 Lighthouse scores"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
