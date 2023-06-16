#[cfg(test)]
pub mod html_basic_generator_headers_tests {
    use crate::common;

    use rs_blog::{
        html_generator::HtmlGenerator, html_visitor::basic_html_generator::BasicHtmlVisitor,
    };

    #[test]
    fn test_simple_header_h1() {
        let root_node = common::get_ast_tree("# header 1");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><h1 id=\"header-1\">header 1</h1></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_multiple_header_h1() {
        let content = &common::add_line_returns(vec!["# header 1", "# header 2"]);
        let root_node = common::get_ast_tree(content);
        let html = HtmlGenerator { root_node };

        assert_eq!(
            "<article><h1 id=\"header-1\">header 1</h1>\n<h1 id=\"header-2\">header 2</h1></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_different_types_of_headers() {
        let content = &common::add_line_returns(
            [
                "# header 1",
                "## header 2",
                "### header 3",
                "#### header 4",
                "##### header 5",
                "###### header 6",
            ]
            .to_vec(),
        );
        let root_node = common::get_ast_tree(content);
        let html = HtmlGenerator { root_node };

        assert_eq!(
            common::add_line_returns(
                [
                    "<article><h1 id=\"header-1\">header 1</h1>",
                    "<h2 id=\"header-2\">header 2</h2>",
                    "<h3 id=\"header-3\">header 3</h3>",
                    "<h4>header 4</h4>",
                    "<h5>header 5</h5>",
                    "<h6>header 6</h6></article>"
                ]
                .to_vec()
            ),
            html.generate_html(BasicHtmlVisitor)
        );
    }
}
