#[cfg(test)]
mod html_basic_generator_links_tests {
    use rs_blog::{
        html_generator::HtmlGenerator, html_visitor::basic_html_generator::BasicHtmlVisitor,
    };

    use crate::common;

    #[test]
    fn test_simple_link_without_text() {
        let root_node = common::get_ast_tree("[](https://google.com)");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p><a href=\"https://google.com\">https://google.com</a></p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_simple_link_with_text() {
        let root_node = common::get_ast_tree("[Google website](https://google.com)");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p><a href=\"https://google.com\">Google website</a></p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_simple_link_inline_with_content() {
        let root_node = common::get_ast_tree(
            "This is my paragraph about: [Google website](https://google.com). Nice website !",
        );
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p>This is my paragraph about: <a href=\"https://google.com\">Google website</a>. Nice website !</p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }
}
