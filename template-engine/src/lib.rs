use std::mem::discriminant;
use std::slice::SplitMut;
use std::collections::HashMap;

/// STRUCTURES ///
#[derive(Debug, PartialEq)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    ForTag,
    IfTag,
}

// Result of tokenization of the template string
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}


/// IMPLEMENTATION ///
pub fn get_content_type(input: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input, "{%", "%}");
    let is_for_tag = (check_symbol_string(&input, "for")
        && check_symbol_string(&input, "in"))
        || check_symbol_string(&input, "endfor");
    let is_if_tag = check_symbol_string(&input, "if")
        || check_symbol_string(&input, "endif");

    let is_template_variable = check_matching_pair(&input, "{{", "}}");
    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag)
    } else if is_template_variable {
        let content = get_expression_data(&input);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }

    return_val
}

pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str, symbol: &str, symbol2: &str) -> bool {
    input.contains(symbol) && input.contains(symbol2)
}

pub fn get_expression_data(input: &str) -> ExpressionData {
    let (_h, i) = get_index_for_symbol(input, '{');
    let head = input[0..i].to_string();
    let (_j, k) = get_index_for_symbol(input, '}');
    let variable = input[i + 1 + 1..k].to_string();
    let tail = input[k + 1 + 1..].to_string();

    ExpressionData {
        head: Some(head),
        variable: variable,
        tail: Some(tail),
    }
}

pub fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;

    while let Some((c, d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }

    (does_exist, index)
}

pub fn generate_html_template_var(
    content: ExpressionData,
    context: HashMap<String, String>,
) -> String {
    let mut html = String::new();

    if let Some(h) = content.head {
        html.push_str(&h);
    }

    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val);
    }

    if let Some(t) = content.tail {
        html.push_str(&t);
    }

    html
}


/// TESTS ///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s ="<h1>Hello world</h1>";

        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test() {
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name". to_string(),
            tail: Some(" , welcome".to_string()),
        };

        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}} , welcome")
        );
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %} ,welcome")
        );
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}")
        );
    }

    #[test]
    fn check_symbol_string_test() {
        assert_eq!(true, check_symbol_string("{{Hello}}", "{{"));
    }

    #[test]
    fn check_symbol_pair_test() {
        assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"));
    }

    #[test]
    fn check_get_expression_data_test() {
        let expression_data = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string()),
        };

        assert_eq!(expression_data, get_expression_data("Hi {{name}} ,welcome"));
    }
}