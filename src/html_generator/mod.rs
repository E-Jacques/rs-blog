use getset::Getters;
use markdown::mdast::Node;

use crate::html_visitor::html_visitor::visit::MdVisitor;

#[derive(Getters, Debug)]
pub struct HtmlGenerator {
    pub root_node: Node,
}

impl HtmlGenerator {
    pub fn generate_html(&self, visitor: impl MdVisitor<String>) -> String {
        // self.compile_html()
        visitor.visit_node(&self.root_node)
    }
}
