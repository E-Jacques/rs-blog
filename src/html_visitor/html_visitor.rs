pub mod visit {
    use markdown::mdast::*;

    pub trait MdVisitor<TNode> {
        #[inline]
        fn visit_node(&self, n: &Node) -> TNode {
            match n {
                Node::Root(x) => self.visit_root(x),
                Node::BlockQuote(x) => self.visit_block_quote(x),
                Node::FootnoteDefinition(x) => self.visit_footnote_definition(x),
                Node::MdxJsxFlowElement(x) => self.visit_mdx_jsx_flow_element(x),
                Node::List(x) => self.visit_list(x),
                Node::MdxjsEsm(x) => self.visit_mdxjs_esm(x),
                Node::Toml(x) => self.visit_toml(x),
                Node::Yaml(x) => self.visit_yaml(x),
                Node::Break(x) => self.visit_break(x),
                Node::InlineCode(x) => self.visit_inline_code(x),
                Node::InlineMath(x) => self.visit_inline_math(x),
                Node::Delete(x) => self.visit_delete(x),
                Node::Emphasis(x) => self.visit_emphasis(x),
                Node::MdxTextExpression(x) => self.visit_mdx_text_expression(x),
                Node::FootnoteReference(x) => self.visit_footnote_reference(x),
                Node::Html(x) => self.visit_html(x),
                Node::Image(x) => self.visit_image(x),
                Node::ImageReference(x) => self.visit_image_reference(x),
                Node::MdxJsxTextElement(x) => self.visit_mdx_jsx_text_element(x),
                Node::Link(x) => self.visit_link(x),
                Node::LinkReference(x) => self.visit_link_reference(x),
                Node::Strong(x) => self.visit_strong(x),
                Node::Text(x) => self.visit_text(x),
                Node::Code(x) => self.visit_code(x),
                Node::Math(x) => self.visit_math(x),
                Node::MdxFlowExpression(x) => self.visit_mdx_flow_expression(x),
                Node::Heading(x) => self.visit_heading(x),
                Node::Table(x) => self.visit_table(x),
                Node::ThematicBreak(x) => self.visit_thematic_break(x),
                Node::TableRow(x) => self.visit_table_row(x),
                Node::TableCell(x) => self.visit_table_cell(x),
                Node::ListItem(x) => self.visit_list_item(x),
                Node::Definition(x) => self.visit_definition(x),
                Node::Paragraph(x) => self.visit_paragraph(x),
            }
        }

        fn visit_root(&self, h: &Root) -> TNode;

        // Container:
        /// Block quote.
        fn visit_block_quote(&self, h: &BlockQuote) -> TNode;
        /// Footnote definition.
        fn visit_footnote_definition(&self, h: &FootnoteDefinition) -> TNode;
        /// MDX: JSX element (container).
        fn visit_mdx_jsx_flow_element(&self, h: &MdxJsxFlowElement) -> TNode;
        /// List.
        fn visit_list(&self, h: &List) -> TNode;

        // Frontmatter:
        /// MDX.js ESM.
        fn visit_mdxjs_esm(&self, h: &MdxjsEsm) -> TNode;
        /// Toml.
        fn visit_toml(&self, h: &Toml) -> TNode;
        /// Yaml.
        fn visit_yaml(&self, h: &Yaml) -> TNode;

        // Phrasing:
        /// Break.
        fn visit_break(&self, h: &Break) -> TNode;
        /// Code (phrasing).
        fn visit_inline_code(&self, h: &InlineCode) -> TNode;
        /// Math (phrasing).
        fn visit_inline_math(&self, h: &InlineMath) -> TNode;
        /// Delete.
        fn visit_delete(&self, h: &Delete) -> TNode;
        /// Emphasis.
        fn visit_emphasis(&self, h: &Emphasis) -> TNode;
        // MDX: expression (text).
        fn visit_mdx_text_expression(&self, h: &MdxTextExpression) -> TNode;
        /// Footnote reference.
        fn visit_footnote_reference(&self, h: &FootnoteReference) -> TNode;
        /// Html (phrasing).
        fn visit_html(&self, h: &Html) -> TNode;
        /// Image.
        fn visit_image(&self, h: &Image) -> TNode;
        /// Image reference.
        fn visit_image_reference(&self, h: &ImageReference) -> TNode;
        // MDX: JSX element (text).
        fn visit_mdx_jsx_text_element(&self, h: &MdxJsxTextElement) -> TNode;
        /// Link.
        fn visit_link(&self, h: &Link) -> TNode;
        /// Link reference.
        fn visit_link_reference(&self, h: &LinkReference) -> TNode;
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
        fn visit_mdx_flow_expression(&self, h: &MdxFlowExpression) -> TNode;
        /// Heading.
        fn visit_heading(&self, h: &Heading) -> TNode;
        /// Html (flow).
        /// Table.
        fn visit_table(&self, h: &Table) -> TNode;
        /// Thematic break.
        fn visit_thematic_break(&self, h: &ThematicBreak) -> TNode;

        // Table content.
        /// Table row.
        fn visit_table_row(&self, h: &TableRow) -> TNode;

        // Row content.
        /// Table cell.
        fn visit_table_cell(&self, h: &TableCell) -> TNode;

        // List content.
        /// List item.
        fn visit_list_item(&self, h: &ListItem) -> TNode;

        // Content.
        /// Definition.
        fn visit_definition(&self, h: &Definition) -> TNode;
        /// Paragraph.
        fn visit_paragraph(&self, h: &Paragraph) -> TNode;
    }
}
