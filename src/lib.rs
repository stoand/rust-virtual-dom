#[macro_use]
extern crate stdweb;

pub mod template;
pub mod render;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VirtualDom(pub Vec<VirtualNode>);

impl<'a, T: ToString> From<T> for VirtualDom {
    fn from(s: T) -> VirtualDom {
        VirtualDom(vec![VirtualNode::Text(s.to_string())])
    }
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum VirtualNode {
    Text(String),
    Element(VirtualElement),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VirtualElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub child_nodes: Vec<VirtualNode>,
}

impl VirtualElement {
    pub fn new() -> Self {
        VirtualElement {
            name: "div".to_string(),
            attributes: HashMap::new(),
            child_nodes: Vec::new(),
        }
    }
}

impl fmt::Display for VirtualElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut attr = String::new();
        for (key, val) in &self.attributes {
            attr += &(" ".to_string() + key + "=\"" + val + "\"");
        }

        let mut inner_html = String::new();
        for child_node in &self.child_nodes {
            match child_node {
                &VirtualNode::Text(ref text) => inner_html += &text,
                &VirtualNode::Element(ref element) => inner_html += &element.to_string(),
            };
        }

        f.write_str(&("<".to_string() + &self.name + &attr + ">" + &inner_html + "</" + &self.name + ">"))
    }
}

#[test]
fn virtual_element_to_string() {
        let mut el = ::VirtualElement::new();
        let group_el = ::VirtualElement::new();

        el.name = "form".to_string();
        el.attributes.insert("class".into(), "active red".into());
        el.child_nodes.push(::VirtualNode::Element(group_el));
        assert_eq!(el.to_string(), "<form class=\"active red\"><div></div></form>");
}