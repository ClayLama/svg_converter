
use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct SvgDocument {
    pub root: SvgElement,
}

#[derive(Debug, Serialize)]
pub struct SvgElement {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<SvgElement>,
}

pub fn parse_svg<P: AsRef<Path>>(
    path: P,
) -> Result<SvgDocument, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let document = roxmltree::Document::parse(&content)?;

    let root = parse_element(document.root_element());

    Ok(SvgDocument { root })
}

fn parse_element(node: roxmltree::Node<'_, '_>) -> SvgElement {
    let mut attributes = BTreeMap::new();

    for attribute in node.attributes() {
        let key = match attribute.namespace() {
            Some(namespace) => {
                format!("{{{}}}{}", namespace, attribute.name())
            }
            None => attribute.name().to_string(),
        };

        attributes.insert(key, attribute.value().to_string());
    }

    let children = node
        .children()
        .filter(|child| child.is_element())
        .map(parse_element)
        .collect();

    let text_parts: Vec<&str> = node
        .children()
        .filter(|child| child.is_text())
        .filter_map(|child| child.text())
        .collect();

    let text = if text_parts.is_empty() {
        None
    } else {
        Some(text_parts.concat())
    };

    SvgElement {
        name: node.tag_name().name().to_string(),
        attributes,
        text,
        children,
    }
}

pub fn save_json<P: AsRef<Path>>(
    svg: &SvgDocument,
    path: P,
) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(svg)?;
    fs::write(path, json)?;
    Ok(())
}
