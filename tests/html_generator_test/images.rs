#[cfg(test)]
mod html_basic_generator_images_tests {
    use rs_blog::{
        html_generator::HtmlGenerator, html_visitor::basic_html_generator::BasicHtmlVisitor,
    };

    use crate::common::{self, add_line_returns};

    #[test]
    fn test_basic_image() {
        let root_node = common::get_ast_tree("![](my_images.png)");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p><img src=\"my_images.png\"></p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_image_with_complex_path() {
        let root_node = common::get_ast_tree("![](assets/images/my_images.png)");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p><img src=\"assets/images/my_images.png\"></p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_basic_image_with_alt() {
        let root_node = common::get_ast_tree("![My alt](my_images.png)");
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><p><img alt=\"My alt\" src=\"my_images.png\"></p></article>",
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_multiples_images_with_alt() {
        let content = add_line_returns(vec![
            "![My first alt](my_first_images.png)",
            "![My second alt](my_second_images.png)",
        ]);
        let root_node = common::get_ast_tree(&content);
        let html = HtmlGenerator { root_node };
        assert_eq!(
            add_line_returns(vec![
                "<article><p><img alt=\"My first alt\" src=\"my_first_images.png\">",
                "<img alt=\"My second alt\" src=\"my_second_images.png\"></p></article>",
            ]),
            html.generate_html(BasicHtmlVisitor)
        );
    }

    #[test]
    fn test_multiples_images_with_alt_and_more_lines() {
        let content = add_line_returns(vec![
            "![My first alt](my_first_images.png)",
            "",
            "![My second alt](my_second_images.png)",
        ]);
        let root_node = common::get_ast_tree(&content);
        let html = HtmlGenerator { root_node };
        assert_eq!(
            add_line_returns(vec![
                "<article><p><img alt=\"My first alt\" src=\"my_first_images.png\"></p>",
                "<p><img alt=\"My second alt\" src=\"my_second_images.png\"></p></article>",
            ]),
            html.generate_html(BasicHtmlVisitor)
        );
    }
}
