use markdown::mdast::{Node, Text};
use mdast_util_to_markdown::to_markdown as to;
use pretty_assertions::assert_eq;

#[test]
fn utf8_with_trailing_spaces() {
    let text = r#"# а  
а  
а  
"#;

    let ast = markdown::to_mdast(&text, &markdown::ParseOptions::default()).unwrap();

    assert_eq!(
        to(&ast).unwrap(),
        "",
        "Should support empty trailing spaces"
    );
}
