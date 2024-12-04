use std::sync::LazyLock;

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use syntect::{
    highlighting::{Theme, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};

pub fn markdown_to_html(markdown: &str) -> String {
    static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
    static THEME: LazyLock<Theme> = LazyLock::new(|| {
        let theme_set = ThemeSet::load_defaults();
        theme_set.themes["base16-ocean.dark"].clone()
    });

    let mut sr = SYNTAX_SET.find_syntax_plain_text();
    let mut code = String::new();
    let mut code_block = false;
    let parser = Parser::new(markdown).filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            let lang = lang.trim();
            sr = SYNTAX_SET
                .find_syntax_by_token(lang)
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
            code_block = true;
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            let html =
                highlighted_html_for_string(&code, &SYNTAX_SET, sr, &THEME).unwrap_or(code.clone());
            code.clear();
            code_block = false;
            Some(Event::Html(html.into()))
        }

        Event::Text(t) => {
            if code_block {
                code.push_str(&t);
                return None;
            }
            Some(Event::Text(t))
        }
        _ => Some(event),
    });
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
