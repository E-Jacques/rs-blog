pub mod visit {
    use markdown::mdast::*;

    pub trait MdVisitor<TNode> {
        #[inline]
        fn visit_node(&self, n: &Node) -> TNode {
            match n {
                Node::Root(x) => self.visit_root(x),
                Node::BlockQuote(x) => self.visit_blockQuote(x),
                Node::FootnoteDefinition(x) => self.visit_footnoteDefinition(x),
                Node::MdxJsxFlowElement(x) => self.visit_mdxJsxFlowElement(x),
                Node::List(x) => self.visit_list(x),
                Node::MdxjsEsm(x) => self.visit_mdxjsEsm(x),
                Node::Toml(x) => self.visit_toml(x),
                Node::Yaml(x) => self.visit_yaml(x),
                Node::Break(x) => self.visit_break(x),
                Node::InlineCode(x) => self.visit_inlineCode(x),
                Node::InlineMath(x) => self.visit_inlineMath(x),
                Node::Delete(x) => self.visit_delete(x),
                Node::Emphasis(x) => self.visit_emphasis(x),
                Node::MdxTextExpression(x) => self.visit_mdxTextExpression(x),
                Node::FootnoteReference(x) => self.visit_footnoteReference(x),
                Node::Html(x) => self.visit_html(x),
                Node::Image(x) => self.visit_image(x),
                Node::ImageReference(x) => self.visit_imageReference(x),
                Node::MdxJsxTextElement(x) => self.visit_mdxJsxTextElement(x),
                Node::Link(x) => self.visit_link(x),
                Node::LinkReference(x) => self.visit_linkReference(x),
                Node::Strong(x) => self.visit_strong(x),
                Node::Text(x) => self.visit_text(x),
                Node::Code(x) => self.visit_code(x),
                Node::Math(x) => self.visit_math(x),
                Node::MdxFlowExpression(x) => self.visit_mdxFlowExpression(x),
                Node::Heading(x) => self.visit_heading(x),
                Node::Table(x) => self.visit_table(x),
                Node::ThematicBreak(x) => self.visit_thematicBreak(x),
                Node::TableRow(x) => self.visit_tableRow(x),
                Node::TableCell(x) => self.visit_tableCell(x),
                Node::ListItem(x) => self.visit_listItem(x),
                Node::Definition(x) => self.visit_definition(x),
                Node::Paragraph(x) => self.visit_paragraph(x),
            }
        }

        fn visit_root(&self, h: &Root) -> TNode;

        // Container:
        /// Block quote.
        fn visit_blockQuote(&self, h: &BlockQuote) -> TNode;
        /// Footnote definition.
        fn visit_footnoteDefinition(&self, h: &FootnoteDefinition) -> TNode;
        /// MDX: JSX element (container).
        fn visit_mdxJsxFlowElement(&self, h: &MdxJsxFlowElement) -> TNode;
        /// List.
        fn visit_list(&self, h: &List) -> TNode;

        // Frontmatter:
        /// MDX.js ESM.
        fn visit_mdxjsEsm(&self, h: &MdxjsEsm) -> TNode;
        /// Toml.
        fn visit_toml(&self, h: &Toml) -> TNode;
        /// Yaml.
        fn visit_yaml(&self, h: &Yaml) -> TNode;

        // Phrasing:
        /// Break.
        fn visit_break(&self, h: &Break) -> TNode;
        /// Code (phrasing).
        fn visit_inlineCode(&self, h: &InlineCode) -> TNode;
        /// Math (phrasing).
        fn visit_inlineMath(&self, h: &InlineMath) -> TNode;
        /// Delete.
        fn visit_delete(&self, h: &Delete) -> TNode;
        /// Emphasis.
        fn visit_emphasis(&self, h: &Emphasis) -> TNode;
        // MDX: expression (text).
        fn visit_mdxTextExpression(&self, h: &MdxTextExpression) -> TNode;
        /// Footnote reference.
        fn visit_footnoteReference(&self, h: &FootnoteReference) -> TNode;
        /// Html (phrasing).
        fn visit_html(&self, h: &Html) -> TNode;
        /// Image.
        fn visit_image(&self, h: &Image) -> TNode;
        /// Image reference.
        fn visit_imageReference(&self, h: &ImageReference) -> TNode;
        // MDX: JSX element (text).
        fn visit_mdxJsxTextElement(&self, h: &MdxJsxTextElement) -> TNode;
        /// Link.
        fn visit_link(&self, h: &Link) -> TNode;
        /// Link reference.
        fn visit_linkReference(&self, h: &LinkReference) -> TNode;
        /// Strong
        fn visit_strong(&self, h: &Strong) -> TNode;
        /// Text.
        fn visit_text(&self, h: &Text) -> TNode;

        // Flow:
        /// Code (flow).
        fn visit_code(&self, h: &Code) -> TNode;
        /// Math (flow).
        fn visit_math(&self, h: &Math) -> TNode;
        // MDX: expression (flow).
        fn visit_mdxFlowExpression(&self, h: &MdxFlowExpression) -> TNode;
        /// Heading.
        fn visit_heading(&self, h: &Heading) -> TNode;
        /// Html (flow).
        /// Table.
        fn visit_table(&self, h: &Table) -> TNode;
        /// Thematic break.
        fn visit_thematicBreak(&self, h: &ThematicBreak) -> TNode;

        // Table content.
        /// Table row.
        fn visit_tableRow(&self, h: &TableRow) -> TNode;

        // Row content.
        /// Table cell.
        fn visit_tableCell(&self, h: &TableCell) -> TNode;

        // List content.
        /// List item.
        fn visit_listItem(&self, h: &ListItem) -> TNode;

        // Content.
        /// Definition.
        fn visit_definition(&self, h: &Definition) -> TNode;
        /// Paragraph.
        fn visit_paragraph(&self, h: &Paragraph) -> TNode;
    }
}

use visit::*;

pub fn surround_by_tags(content: String, tag_name: String) -> String {
    let mut s = "<".to_owned();
    s.push_str(&tag_name);
    s.push_str(">");
    s.push_str(&content);
    s.push_str("</");
    s.push_str(&tag_name);
    s.push_str(">");

    return s;
}

pub struct BasicHtmlGenerator;
impl MdVisitor<String> for BasicHtmlGenerator {
    fn visit_root(&self, h: &markdown::mdast::Root) -> String {
        let content = h
            .children
            .clone()
            .into_iter()
            .map(|child| self.visit_node(&child))
            .collect::<Vec<String>>()
            .join("\n");

        return surround_by_tags(content, String::from("article"));
    }

    fn visit_blockQuote(&self, h: &markdown::mdast::BlockQuote) -> String {
        return String::from("");
    }

    fn visit_footnoteDefinition(&self, h: &markdown::mdast::FootnoteDefinition) -> String {
        return String::from("");
    }

    fn visit_mdxJsxFlowElement(&self, h: &markdown::mdast::MdxJsxFlowElement) -> String {
        return String::from("");
    }

    fn visit_list(&self, h: &markdown::mdast::List) -> String {
        return String::from("");
    }

    fn visit_mdxjsEsm(&self, h: &markdown::mdast::MdxjsEsm) -> String {
        return String::from("");
    }

    fn visit_toml(&self, h: &markdown::mdast::Toml) -> String {
        return String::from("");
    }

    fn visit_yaml(&self, h: &markdown::mdast::Yaml) -> String {
        return String::from("");
    }

    fn visit_break(&self, h: &markdown::mdast::Break) -> String {
        return String::from("");
    }

    fn visit_inlineCode(&self, h: &markdown::mdast::InlineCode) -> String {
        return String::from("");
    }

    fn visit_inlineMath(&self, h: &markdown::mdast::InlineMath) -> String {
        return String::from("");
    }

    fn visit_delete(&self, h: &markdown::mdast::Delete) -> String {
        return String::from("");
    }

    fn visit_emphasis(&self, h: &markdown::mdast::Emphasis) -> String {
        return String::from("");
    }

    fn visit_mdxTextExpression(&self, h: &markdown::mdast::MdxTextExpression) -> String {
        return String::from("");
    }

    fn visit_footnoteReference(&self, h: &markdown::mdast::FootnoteReference) -> String {
        return String::from("");
    }

    fn visit_html(&self, h: &markdown::mdast::Html) -> String {
        return String::from("");
    }

    fn visit_image(&self, h: &markdown::mdast::Image) -> String {
        return String::from("");
    }

    fn visit_imageReference(&self, h: &markdown::mdast::ImageReference) -> String {
        return String::from("");
    }

    fn visit_mdxJsxTextElement(&self, h: &markdown::mdast::MdxJsxTextElement) -> String {
        return String::from("");
    }

    fn visit_link(&self, h: &markdown::mdast::Link) -> String {
        return String::from("");
    }

    fn visit_linkReference(&self, h: &markdown::mdast::LinkReference) -> String {
        return String::from("");
    }

    fn visit_strong(&self, h: &markdown::mdast::Strong) -> String {
        return String::from("");
    }

    fn visit_text(&self, h: &markdown::mdast::Text) -> String {
        return h.value.clone();
    }

    fn visit_code(&self, h: &markdown::mdast::Code) -> String {
        return String::from("");
    }

    fn visit_math(&self, h: &markdown::mdast::Math) -> String {
        return String::from("");
    }

    fn visit_mdxFlowExpression(&self, h: &markdown::mdast::MdxFlowExpression) -> String {
        return String::from("");
    }

    fn visit_heading(&self, h: &markdown::mdast::Heading) -> String {
        let content = h
            .children
            .clone()
            .into_iter()
            .map(|child| self.visit_node(&child))
            .collect::<Vec<String>>()
            .join(" ");

        let tag_name = format!("h{}", h.depth);
        dbg!(format!("content: {}, tag_name: {}", content, tag_name));

        return surround_by_tags(content, tag_name);
    }

    fn visit_table(&self, h: &markdown::mdast::Table) -> String {
        return String::from("");
    }

    fn visit_thematicBreak(&self, h: &markdown::mdast::ThematicBreak) -> String {
        return String::from("");
    }

    fn visit_tableRow(&self, h: &markdown::mdast::TableRow) -> String {
        return String::from("");
    }

    fn visit_tableCell(&self, h: &markdown::mdast::TableCell) -> String {
        return String::from("");
    }

    fn visit_listItem(&self, h: &markdown::mdast::ListItem) -> String {
        return String::from("");
    }

    fn visit_definition(&self, h: &markdown::mdast::Definition) -> String {
        return String::from("");
    }

    fn visit_paragraph(&self, h: &markdown::mdast::Paragraph) -> String {
        return String::from("");
    }
}
