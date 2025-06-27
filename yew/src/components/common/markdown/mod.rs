use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

/// Props for the Markdown component
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct MarkdownProps {
    /// The markdown content to render
    pub content: String,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

impl Default for MarkdownProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            class: Classes::new(),
        }
    }
}

/// A component that renders markdown content as HTML
#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let html_content = parse_markdown_to_html(&props.content);

    html! {
        <div class={props.class.clone()}
             dangerously_set_inner_html={html_content}>
        </div>
    }
}

/// Parse markdown content to HTML using pulldown-cmark, with no custom styling
pub fn parse_markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    pub mod accessibility;
    pub mod edge_cases;
    pub mod interactions;
    pub mod props;
    pub mod rendering;
    pub mod variants;

    use super::*;
    use wasm_bindgen_test::*;
    use yew::platform::spawn_local;

    wasm_bindgen_test_configure!(run_in_browser);
}
