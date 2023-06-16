use std::collections::HashMap;

pub enum HTMLAttributesValues<'a> {
    String(&'a str),
    VecOfString(Vec<&'a str>),
}

pub fn surround_by_tags(content: &str, tag_name: &str) -> String {
    surround_by_tags_with_details(content, tag_name, None, None)
}

pub fn to_kebab_case(value: &str) -> String {
    value.clone().replace(" ", "-").to_lowercase()
}

pub fn surround_by_tags_with_details(
    content: &str,
    tag_name: &str,
    id: Option<&str>,
    classes: Option<Vec<&str>>,
) -> String {
    let mut hash_map = HashMap::new();

    match id {
        Some(id_value) => {
            hash_map.insert("id", HTMLAttributesValues::String(id_value));
        }
        _ => (),
    }

    match classes {
        Some(classes_value) => {
            hash_map.insert("class", HTMLAttributesValues::VecOfString(classes_value));
        }
        _ => (),
    }

    surround_by_tags_with_attributes(content, tag_name, &hash_map, true)
}

pub fn surround_by_tags_with_attributes(
    content: &str,
    tag_name: &str,
    attributes: &HashMap<&str, HTMLAttributesValues>,
    have_closing_tags: bool,
) -> String {
    let mut s = "<".to_owned();
    s.push_str(tag_name);

    let mut sorted_attributes: Vec<_> = attributes.iter().collect();
    sorted_attributes.sort_by_key(|keyvalue| keyvalue.0);

    for (key, value) in sorted_attributes {
        match value {
            HTMLAttributesValues::String(v) => {
                if v.len() > 0 {
                    s.push_str(&format!(" {key}=\"{v}\""))
                }
            }
            HTMLAttributesValues::VecOfString(v) => {
                let formated_attr_vec = v
                    .clone()
                    .into_iter()
                    .filter(|c| c.len() > 0)
                    .collect::<Vec<_>>()
                    .join(" ");

                if formated_attr_vec.len() > 0 {
                    s.push_str(&format!(" {key}=\"{formated_attr_vec}\""))
                }
            }
        }
    }

    s.push_str(">");

    if have_closing_tags {
        s.push_str(content);
        s.push_str("</");
        s.push_str(tag_name);
        s.push_str(">");
    }

    return s;
}

pub fn multiple_spaces_into_one(s: &str) -> String {
    dbg!(s);
    if s.len() == 0 {
        return String::from("");
    }

    for char in s.chars() {
        if char != ' ' {
            return String::from(s);
        }
    }

    String::from(" ")
}

#[cfg(test)]
mod helpers_tests {
    use std::collections::HashMap;

    use crate::html_visitor::helpers::{
        multiple_spaces_into_one, surround_by_tags_with_details, to_kebab_case,
    };

    use super::{surround_by_tags, surround_by_tags_with_attributes, HTMLAttributesValues};

    #[test]
    fn test_surround_by_tag() {
        let result = surround_by_tags("My content", "my-tag");
        assert_eq!(result, "<my-tag>My content</my-tag>");
    }

    #[test]
    fn test_nested_surround_by_tag() {
        let child = surround_by_tags("child content", "child");
        let parent = surround_by_tags(&child, "parent");

        assert_eq!(parent, "<parent><child>child content</child></parent>");
    }

    #[test]
    fn test_surround_by_tag_with_id() {
        let result = surround_by_tags_with_details("My content", "my-tag", Some("my-id"), None);
        assert_eq!(result, "<my-tag id=\"my-id\">My content</my-tag>");
    }

    #[test]
    fn test_surround_by_tag_with_one_class() {
        let result =
            surround_by_tags_with_details("My content", "my-tag", None, Some(vec!["class-1"]));
        assert_eq!(result, "<my-tag class=\"class-1\">My content</my-tag>");
    }

    #[test]
    fn test_surround_by_tag_with_multiple_classes() {
        let result = surround_by_tags_with_details(
            "My content",
            "my-tag",
            None,
            Some(vec!["class-1", "class-2", "class-3"]),
        );
        assert_eq!(
            result,
            "<my-tag class=\"class-1 class-2 class-3\">My content</my-tag>"
        );
    }

    #[test]
    fn test_surround_by_tag_with_one_class_and_id() {
        let result = surround_by_tags_with_details(
            "My content",
            "my-tag",
            Some("my-id"),
            Some(vec!["class-1"]),
        );
        assert_eq!(
            result,
            "<my-tag class=\"class-1\" id=\"my-id\">My content</my-tag>"
        );
    }

    #[test]
    fn test_surround_by_tag_with_multiple_classes_and_id() {
        let result = surround_by_tags_with_details(
            "My content",
            "my-tag",
            Some("my-id"),
            Some(vec!["class-1", "class-2", "class-3"]),
        );
        assert_eq!(
            result,
            "<my-tag class=\"class-1 class-2 class-3\" id=\"my-id\">My content</my-tag>"
        );
    }

    #[test]
    fn test_surround_by_tag_without_specified_attr() {
        let attributes = HashMap::new();

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, true);
        assert_eq!(result, "<tag-name>content</tag-name>");
    }

    #[test]
    fn test_surround_by_tag_with_one_attribute() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, true);
        assert_eq!(result, "<tag-name href=\"link\">content</tag-name>");
    }

    #[test]
    fn test_surround_by_tag_dont_display_empty_string() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::String(""));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, true);
        assert_eq!(result, "<tag-name href=\"link\">content</tag-name>");
    }

    #[test]
    fn test_surround_by_tag_dont_display_empty_vec_of_string() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::VecOfString(vec![]));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, true);
        assert_eq!(result, "<tag-name href=\"link\">content</tag-name>");
    }

    #[test]
    fn test_surround_by_tag_with_multiple_attributes() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::String("my-id"));
        attributes.insert(
            "class",
            HTMLAttributesValues::VecOfString(vec!["class-1", "class-2", "class-3", ""]),
        );

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, true);
        assert_eq!(
            result,
            "<tag-name class=\"class-1 class-2 class-3\" href=\"link\" id=\"my-id\">content</tag-name>"
        );
    }

    #[test]
    fn test_surround_by_tag_without_specified_attr_without_closing_tag() {
        let attributes = HashMap::new();

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, false);
        assert_eq!(result, "<tag-name>");
    }

    #[test]
    fn test_surround_by_tag_with_one_attribute_without_closing_tag() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, false);
        assert_eq!(result, "<tag-name href=\"link\">");
    }

    #[test]
    fn test_surround_by_tag_dont_display_empty_string_without_closing_tag() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::String(""));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, false);
        assert_eq!(result, "<tag-name href=\"link\">");
    }

    #[test]
    fn test_surround_by_tag_dont_display_empty_vec_of_string_without_closing_tag() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::VecOfString(vec![]));

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, false);
        assert_eq!(result, "<tag-name href=\"link\">");
    }

    #[test]
    fn test_surround_by_tag_with_multiple_attributes_without_closing_tag() {
        let mut attributes = HashMap::new();
        attributes.insert("href", HTMLAttributesValues::String("link"));
        attributes.insert("id", HTMLAttributesValues::String("my-id"));
        attributes.insert(
            "class",
            HTMLAttributesValues::VecOfString(vec!["class-1", "class-2", "class-3", ""]),
        );

        let result = surround_by_tags_with_attributes("content", "tag-name", &attributes, false);
        assert_eq!(
            result,
            "<tag-name class=\"class-1 class-2 class-3\" href=\"link\" id=\"my-id\">"
        );
    }

    #[test]
    fn test_to_kebab_case_no_change_on_kebab_case() {
        assert_eq!(to_kebab_case("value"), "value");
        assert_eq!(to_kebab_case("my-value"), "my-value")
    }

    #[test]
    fn test_to_kebab_case_correctly_handle_content() {
        assert_eq!(to_kebab_case("my value"), "my-value");
        assert_eq!(to_kebab_case("My value"), "my-value");
        assert_eq!(
            to_kebab_case("I have MultipLe WordS wiTH DiffeRENT CAps"),
            "i-have-multiple-words-with-different-caps"
        );
    }

    #[test]
    fn test_multiple_spaces_into_one() {
        assert_eq!(multiple_spaces_into_one(""), "");
        assert_eq!(multiple_spaces_into_one("      "), " ");
        assert_eq!(multiple_spaces_into_one(" "), " ");
        assert_eq!(
            multiple_spaces_into_one("      Well... no"),
            "      Well... no"
        );
    }
}
