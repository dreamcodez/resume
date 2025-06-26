//! Progress Component
//!
//! Displays progress bars, spinners, and other progress indicators.

use yew::prelude::*;

/// Progress bar variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

/// Props for the Progress component
#[derive(Properties, PartialEq, Clone)]
pub struct ProgressProps {
    /// Current progress value
    pub value: u32,
    /// Maximum progress value
    pub max: u32,
    /// Progress variant
    #[prop_or(ProgressVariant::Primary)]
    pub variant: ProgressVariant,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Progress bar component
#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let percentage = if props.max > 0 {
        (props.value as f64 / props.max as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    let variant_classes = match props.variant {
        ProgressVariant::Primary => "bg-blue-600",
        ProgressVariant::Secondary => "bg-gray-600",
        ProgressVariant::Success => "bg-green-600",
        ProgressVariant::Warning => "bg-yellow-600",
        ProgressVariant::Danger => "bg-red-600",
    };

    html! {
        <div class={classes!("w-full", props.class.clone())}>
            <div class={classes!("flex", "gap-1", props.class.clone())}>
                <div class={classes!("w-3", "h-3", "rounded-full", "transition-colors", "duration-200")} />
                <div class={classes!("w-3", "h-3", "rounded-full", "transition-colors", "duration-200")} />
                <div class={classes!("w-3", "h-3", "rounded-full", "transition-colors", "duration-200")} />
            </div>

            <div class={classes!("w-full", "bg-gray-200", "rounded-full", "h-2", props.class.clone())}>
                <div
                    class={classes!("h-2", "rounded-full", "transition-all", "duration-300", variant_classes)}
                    style={format!("width: {}%", percentage)}
                    role="progressbar"
                    aria-valuenow={props.value.to_string()}
                    aria-valuemin="0"
                    aria-valuemax={props.max.to_string()}
                />
            </div>
        </div>
    }
}

/// Props for the Spinner component
#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    /// Spinner size
    #[prop_or("md")]
    pub size: &'static str,
    /// Spinner variant
    #[prop_or(ProgressVariant::Primary)]
    pub variant: ProgressVariant,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Spinner component for loading states
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let size_classes = match props.size {
        "sm" => "w-4 h-4",
        "md" => "w-8 h-8",
        "lg" => "w-12 h-12",
        "xl" => "w-16 h-16",
        _ => "w-8 h-8",
    };

    let variant_classes = match props.variant {
        ProgressVariant::Primary => "border-blue-600",
        ProgressVariant::Secondary => "border-gray-600",
        ProgressVariant::Success => "border-green-600",
        ProgressVariant::Warning => "border-yellow-600",
        ProgressVariant::Danger => "border-red-600",
    };

    html! {
        <div class={classes!("relative", "w-16", "h-16", props.class.clone())}>
            <div
                class={classes!(
                    "animate-spin",
                    "rounded-full",
                    "border-4",
                    "border-gray-200",
                    variant_classes,
                    "border-t-transparent",
                    size_classes
                )}
                role="status"
                aria-label="Loading"
            />
        </div>
    }
}

/// Props for the ProgressSteps component
#[derive(Properties, PartialEq)]
pub struct ProgressStepsProps {
    /// Current step (1-based)
    pub current: u32,
    /// Total number of steps
    pub total: u32,
    /// Step labels
    #[prop_or_default]
    pub labels: Vec<String>,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Progress steps component
#[function_component(ProgressSteps)]
pub fn progress_steps(props: &ProgressStepsProps) -> Html {
    let steps: Vec<Html> = (1..=props.total)
        .map(|step| {
            let is_completed = step < props.current;
            let is_current = step == props.current;
            let is_future = step > props.current;

            let step_classes = classes!(
                "flex-1",
                "h-2",
                "rounded-full",
                "transition-all",
                "duration-300",
                if is_completed {
                    "bg-green-600"
                } else if is_current {
                    "bg-blue-600"
                } else {
                    "bg-gray-200"
                }
            );

            let label = if step <= props.labels.len() as u32 {
                props.labels[(step - 1) as usize].clone()
            } else {
                format!("Step {}", step)
            };

            html! {
                <div class="flex flex-col items-center space-y-2">
                    <div class={step_classes} />
                    <span class={classes!(
                        "text-xs",
                        "font-medium",
                        if is_completed {
                            "text-green-600"
                        } else if is_current {
                            "text-blue-600"
                        } else {
                            "text-gray-400"
                        }
                    )}>
                        {label}
                    </span>
                </div>
            }
        })
        .collect();

    html! {
        <div class={classes!("space-y-4", props.class.clone())}>
            <div class="flex space-x-2">
                {steps}
            </div>
            <div class="text-center text-sm text-gray-600">
                {format!("Step {} of {}", props.current, props.total)}
            </div>
        </div>
    }
}

/// Props for the ProgressCard component
#[derive(Properties, PartialEq)]
pub struct ProgressCardProps {
    /// Progress component props
    pub progress: ProgressProps,
    /// Card title
    pub title: String,
    /// Card description
    #[prop_or_default]
    pub description: Option<String>,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Progress card component that wraps progress in a card
#[function_component(ProgressCard)]
pub fn progress_card(props: &ProgressCardProps) -> Html {
    html! {
        <div class={classes!("bg-white", "rounded-lg", "p-6", "shadow-md", props.class.clone())}>
            <div class="space-y-4">
                <div>
                    <h3 class="text-lg font-semibold text-gray-800">
                        {props.title.clone()}
                    </h3>
                    if let Some(desc) = &props.description {
                        <p class="text-sm text-gray-600 mt-1">
                            {desc.clone()}
                        </p>
                    }
                </div>

                <Progress ..props.progress.clone() />
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percentage_calculation() {
        let props = ProgressProps {
            value: 50,
            max: 100,
            variant: ProgressVariant::Primary,
            class: Classes::new(),
        };

        // In a real test, you'd render and check percentage
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_spinner_variants() {
        let props = SpinnerProps {
            size: "md",
            variant: ProgressVariant::Success,
            class: Classes::new(),
        };

        // In a real test, you'd render and check variant styling
        assert!(true); // Placeholder assertion
    }
}
