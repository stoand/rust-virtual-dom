use std::collections::HashMap;

pub struct VirtualDom<'a>(Vec<VirtualNode<'a>>);

impl<'a> VirtualDom<'a> {
    fn nodes(self) -> Vec<VirtualNode<'a>> {
        self.0
    }
}

impl<'a, T: ToString> From<T> for VirtualDom<'a> {
    fn from(s: T) -> VirtualDom<'a> {
        VirtualDom(vec![VirtualNode::Text(&s.to_string())])
    }
}

pub enum VirtualNode<'a> {
    Text(&'a str),
    Element(VirtualElement<'a>),
}

struct VirtualElement<'a> {
    name: &'a str,
    id: Option<&'a str>,
    class: Vec<&'a str>,
    attributes: HashMap<&'a str, &'a str>,
    parentNode: Option<&'a VirtualElement<'a>>,
    childNodes: VirtualDom<'a>,
}

impl<'a> VirtualElement<'a> {
    fn new() -> Self {
        VirtualElement {
            name: "",
            id: None,
            class: Vec::new(),
            attributes: HashMap::new(),
            parentNode: None,
            childNodes: VirtualDom(Vec::new()),
        }
    }
}

macro_rules! template {
    ($($inner:tt)*) => ({
        let mut el = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el);
        el
    });
}

macro_rules! inner_template {
    () => (|el: &mut::VirtualElement| {});
    ([$($key:ident=$val:expr)*]$($inner:tt)*) => (|el: &mut::VirtualElement| {
        $(el.attributes.insert(stringify!($key), $val);)*
        inner_template!($($inner)*)(&mut el);
    });
    (@$bind:expr) => (|el: &mut::VirtualElement|
        el.childNodes.nodes().append(&mut ::VirtualDom::from($bind).nodes()));
    (>($($inner_parens:tt)*)$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_parens);
        el.childNodes.nodes().push(::VirtualNode::Element(el_parens));

        let mut el_remaining = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_remaining);
        el.childNodes.nodes().push(::VirtualNode::Element(el_remaining));
    });
    (>$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_remaining);
        el.childNodes.nodes().push(::VirtualNode::Element(el_remaining));
    });
    (+($($inner_parens:tt)*)$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_parens);
        if let Some(parentNode) = el.parentNode {
            parentNode.childNodes.nodes().push(::VirtualNode::Element(el_parens));
        }

        let mut el_remaining = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_remaining);
        if let Some(parentNode) = el.parentNode {
            parentNode.childNodes.nodes().push(::VirtualNode::Element(el_remaining));
        }
    });
    (+$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_remaining);
        if let Some(parentNode) = el.parentNode {
            parentNode.childNodes.nodes().push(::VirtualNode::Element(el_remaining));
        }
    });
    (.$class:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.class.push(stringify!($class));
        inner_template!($($inner)*)(el);
    });
    (#$id:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.id = Some(stringify!($id));
        inner_template!($($inner)*)(el);
    });
    ($name:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.name = stringify!($name);
        inner_template!($($inner)*)(el);
    });
}

// ".user>(.name-container>.name>@name)+(.views>(span>t_views)+(span>@views))+(.videos>@videos)"
struct User {
    name: String,
    views: i32,
    // videos: Vec<Video>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn template_macro() {
        let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        assert_eq!(t, 1);
    }
}
