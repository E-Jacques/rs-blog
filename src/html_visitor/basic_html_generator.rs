use std::collections::HashMap;

use crate::html_visitor::helpers::{surround_by_tags_with_details, to_kebab_case};

use super::{
    helpers::{
        multiple_spaces_into_one, surround_by_tags, surround_by_tags_with_attributes,
        HTMLAttributesValues,
    },
    html_visitor::visit::MdVisitor,
};

pub struct BasicHtmlVisitor;
impl MdVisitor<String> for BasicHtmlVisitor {
    fn visit_root(&self, h: &markdown::mdast::Root) -> String {
        let content = h
            .children
            .clone()
            .into_iter()
            .map(|child| self.visit_node(&child))
            .collect::<Vec<String>>()
            .join("\n");

        return surround_by_tags(&content, "article");
    }

    fn visit_block_quote(&self, _h: &markdown::mdast::BlockQuote) -> String {
        return String::from("");
    }

    fn visit_footnote_definition(&self, _h: &markdown::mdast::FootnoteDefinition) -> String {
        return String::from("");
    }

    fn visit_mdx_jsx_flow_element(&self, _h: &markdown::mdast::MdxJsxFlowElement) -> String {
        return String::from("");
    }

    fn visit_list(&self, _h: &markdown::mdast::List) -> String {
        return String::from("");
    }

    fn visit_mdxjs_esm(&self, _h: &markdown::mdast::MdxjsEsm) -> String {
        return String::from("");
    }

    fn visit_toml(&self, _h: &markdown::mdast::Toml) -> String {
        return String::from("");
    }

    fn visit_yaml(&self, _h: &markdown::mdast::Yaml) -> String {
        return String::from("");
    }

    fn visit_break(&self, _h: &markdown::mdast::Break) -> String {
        return String::from("");
    }

    fn visit_inline_code(&self, _h: &markdown::mdast::InlineCode) -> String {
        return String::from("");
    }

    fn visit_inline_math(&self, _h: &markdown::mdast::InlineMath) -> String {
        return String::from("");
    }

    fn visit_delete(&self, _h: &markdown::mdast::Delete) -> String {
        return String::from("");
    }

    fn visit_emphasis(&self, _h: &markdown::mdast::Emphasis) -> String {
        return String::from("");
    }

    fn visit_mdx_text_expression(&self, _h: &markdown::mdast::MdxTextExpression) -> String {
        return String::from("");
    }

    fn visit_footnote_reference(&self, _h: &markdown::mdast::FootnoteReference) -> String {
        return String::from("");
    }

    fn visit_html(&self, _h: &markdown::mdast::Html) -> String {
        return String::from("");
    }

    fn visit_image(&self, h: &markdown::mdast::Image) -> String {
        let mut attributes = HashMap::new();
        attributes.insert("src", HTMLAttributesValues::String(&h.url));
        attributes.insert("alt", HTMLAttributesValues::String(&h.alt));

        surround_by_tags_with_attributes("", "img", &attributes, false)
    }

    fn visit_image_reference(&self, _h: &markdown::mdast::ImageReference) -> String {
        return String::from("");
    }

    fn visit_mdx_jsx_text_element(&self, _h: &markdown::mdast::MdxJsxTextElement) -> String {
        return String::from("");
    }

    fn visit_link(&self, h: &markdown::mdast::Link) -> String {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String(&h.url));

        let mut content = h
            .children
            .clone()
            .into_iter()
            .map(|child| self.visit_node(&child))
            .collect::<Vec<String>>()
            .join(" ");

        if content.len() == 0 {
            content = h.url.clone();
        }

        surround_by_tags_with_attributes(&content, "a", &attributes, true)
    }

    fn visit_link_reference(&self, _h: &markdown::mdast::LinkReference) -> String {
        return String::from("");
    }

    fn visit_strong(&self, _h: &markdown::mdast::Strong) -> String {
        return String::from("");
    }

    fn visit_text(&self, h: &markdown::mdast::Text) -> String {
        return h.value.clone();
    }

    fn visit_code(&self, _h: &markdown::mdast::Code) -> String {
        return String::from("");
    }

    fn visit_math(&self, _h: &markdown::mdast::Math) -> String {
        return String::from("");
    }

    fn visit_mdx_flow_expression(&self, _h: &markdown::mdast::MdxFlowExpression) -> String {
        return String::from("");
    }

    fn visit_heading(&self, h: &markdown::mdast::Heading) -> String {
        const LOWEST_DEPTH_FOR_ID: u8 = 3;

        let content = h
            .children
            .clone()
            .into_iter()
            .map(|child| self.visit_node(&child))
            .collect::<Vec<String>>()
            .join(" ");

        let tag_name = format!("h{}", h.depth);
        dbg!(format!("content: {}, tag_name: {}", content, tag_name));

        if h.depth <= LOWEST_DEPTH_FOR_ID {
            surround_by_tags_with_details(&content, &tag_name, Some(&to_kebab_case(&content)), None)
        } else {
            surround_by_tags(&content, &tag_name)
        }
    }

    fn visit_table(&self, _h: &markdown::mdast::Table) -> String {
        return String::from("");
    }

    fn visit_thematic_break(&self, _h: &markdown::mdast::ThematicBreak) -> String {
        return String::from("");
    }

    fn visit_table_row(&self, _h: &markdown::mdast::TableRow) -> String {
        return String::from("");
    }

    fn visit_table_cell(&self, _h: &markdown::mdast::TableCell) -> String {
        return String::from("");
    }

    fn visit_list_item(&self, _h: &markdown::mdast::ListItem) -> String {
        return String::from("");
    }

    fn visit_definition(&self, _h: &markdown::mdast::Definition) -> String {
        return String::from("");
    }

    fn visit_paragraph(&self, h: &markdown::mdast::Paragraph) -> String {
        let content = h
            .children
            .clone()
            .into_iter()
            .map(|child| multiple_spaces_into_one(&self.visit_node(&child)))
            .collect::<Vec<String>>()
            .join("");

        return surround_by_tags(&content, "p");
    }
}
