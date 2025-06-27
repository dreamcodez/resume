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

/// A component that renders markdown content with custom styling
#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let html_content = parse_markdown_to_html(&props.content);

    html! {
        <div class={classes!("prose", "prose-sm", "max-w-none", props.class.clone())}>
            <div class="whitespace-pre-line text-gray-700 leading-relaxed"
                 dangerously_set_inner_html={html_content}>
            </div>
        </div>
    }
}

/// Parse markdown content to HTML using pulldown-cmark
fn parse_markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Apply custom styling to the generated HTML
    apply_custom_styling(&html_output)
}

/// Apply custom styling to the generated HTML
fn apply_custom_styling(html: &str) -> String {
    html.replace("<ul>", "<ul class=\"ml-4 mb-2\">")
        .replace("<li>", "<li class=\"mb-1\">")
        .replace("<p>", "<p class=\"mb-2\">")
        .replace("<strong>", "<span class=\"font-semibold text-gray-900\">")
        .replace("</strong>", "</span>")
        .replace("<em>", "<span class=\"italic text-gray-700\">")
        .replace("</em>", "</span>")
        .replace(
            "<code>",
            "<code class=\"bg-gray-100 px-1 py-0.5 rounded text-sm font-mono\">",
        )
        .replace("</code>", "</code>")
        .replace(
            "<pre>",
            "<pre class=\"bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2\">",
        )
        .replace("</pre>", "</pre>")
        .replace(
            "<h1>",
            "<h1 class=\"text-2xl font-bold mb-4 text-gray-900\">",
        )
        .replace("</h1>", "</h1>")
        .replace(
            "<h2>",
            "<h2 class=\"text-xl font-semibold mb-3 text-gray-900\">",
        )
        .replace("</h2>", "</h2>")
        .replace(
            "<h3>",
            "<h3 class=\"text-lg font-medium mb-2 text-gray-900\">",
        )
        .replace("</h3>", "</h3>")
        .replace(
            "<h4>",
            "<h4 class=\"text-base font-medium mb-2 text-gray-900\">",
        )
        .replace("</h4>", "</h4>")
        .replace(
            "<h5>",
            "<h5 class=\"text-sm font-medium mb-1 text-gray-900\">",
        )
        .replace("</h5>", "</h5>")
        .replace(
            "<h6>",
            "<h6 class=\"text-xs font-medium mb-1 text-gray-900\">",
        )
        .replace("</h6>", "</h6>")
        .replace(
            "<blockquote>",
            "<blockquote class=\"border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2\">",
        )
        .replace("</blockquote>", "</blockquote>")
        .replace(
            "<a ",
            "<a class=\"text-blue-600 hover:text-blue-800 underline\" ",
        )
        .replace(
            "<table>",
            "<table class=\"border-collapse border border-gray-300 mb-2\">",
        )
        .replace(
            "<th>",
            "<th class=\"border border-gray-300 px-3 py-2 bg-gray-100 font-semibold\">",
        )
        .replace("<td>", "<td class=\"border border-gray-300 px-3 py-2\">")
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
