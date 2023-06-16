#[cfg(test)]
mod html_basic_generator_paragraph_tests {
    use crate::common;

    use rs_blog::{
        html_generator::HtmlGenerator, html_visitor::basic_html_generator::BasicHtmlVisitor,
    };

    #[test]
    fn test_paragraph() {
        let root_node = common::get_ast_tree("This is my paragraph");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p>This is my paragraph</p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_multiple_lines_paragraph() {
        let content = common::add_line_returns(vec![
            "This is my first paragraph",
            "This is my second paragraph",
        ]);
        let root_node = common::get_ast_tree(&content);
        let html = HtmlGenerator { root_node };
        assert_eq!(
            common::add_line_returns(vec![
                "<article><p>This is my first paragraph",
                "This is my second paragraph</p></article>"
            ]),
            html.generate_html(BasicHtmlVisitor)
        )
    }

    #[test]
    fn test_multiple_paragraphs() {
        let content = common::add_line_returns(vec![
            "This is my first paragraph",
            "",
            "This is my second paragraph",
        ]);
        let root_node = common::get_ast_tree(&content);
        let html = HtmlGenerator { root_node };
        assert_eq!(
            common::add_line_returns(vec![
                "<article><p>This is my first paragraph</p>",
                "<p>This is my second paragraph</p></article>"
            ]),
            html.generate_html(BasicHtmlVisitor)
        )
    }
}
