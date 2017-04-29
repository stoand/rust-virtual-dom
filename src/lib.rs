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
    childNodes: VirtualDom<'a>,
}

impl<'a> VirtualElement<'a> {
    fn new() -> Self {
        VirtualElement {
            name: "",
            id: None,
            class: Vec::new(),
            attributes: HashMap::new(),
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
    ([$( $key:ident=$val:expr)*]$($inner:tt)*) => (|el: &mut::VirtualElement| {
        $(el.attributes.insert(stringify!($key), $val);)*
        inner_template!($($inner)*)(&mut el);
    });
    (@$bind:expr) => (|el: &mut::VirtualElement|
        el.childNodes.nodes().append(&mut ::VirtualDom::from($bind).nodes()));
    (>($($inner:tt)*)$($inner1:tt)*) => (|el: &mut::VirtualElement| {
        inner_template!($($inner)*);
    });
    (>$($inner:tt)*) => (inner_template!($($inner)*));
    (+$($inner:tt)*) => (inner_template!($($inner)*));
    (.$class:ident$($inner:tt)*) => (inner_template!($($inner)*));
    (#$id:ident$($inner:tt)*) => (inner_template!($($inner)*));
    ($name:ident$($inner:tt)*) => (inner_template!($($inner)*));
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
        let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a);
        assert_eq!(t, 1);
    }
}
