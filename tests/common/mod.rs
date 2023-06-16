use markdown::{mdast::Node, to_mdast, ParseOptions};

pub fn get_ast_tree(content: &str) -> Node {
    let ast_result = to_mdast(content, &ParseOptions::default());
    ast_result.unwrap()
}

pub fn add_line_returns(lines: Vec<&str>) -> String {
    let content = lines.join("\n");
    content
}
