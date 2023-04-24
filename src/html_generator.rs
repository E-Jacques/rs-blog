use getset::Getters;
use markdown::mdast::Node;

use crate::html_visitor::{visit::MdVisitor, BasicHtmlGenerator};

#[derive(Getters, Debug)]
pub struct HtmlGenerator {
    root_node: Node,
}

impl HtmlGenerator {
    #[inline]
    pub fn generate_html(&self, visitor: impl MdVisitor<String>) -> String {
        // self.compile_html()
        visitor.visit_node(&self.root_node)
    }
}

#[cfg(test)]
mod test {
    use markdown::{to_mdast, ParseOptions};

    use crate::html_visitor::BasicHtmlGenerator;

    use super::HtmlGenerator;

    #[test]
    fn test_simple_header() {
        let ast_result = to_mdast("# header 1", &ParseOptions::default());
        let root_node = ast_result.unwrap();
        let html = HtmlGenerator { root_node };
        assert_eq!(
            "<article><h1>header 1</h1></article>",
            html.generate_html(BasicHtmlGenerator)
        );
    }
}
