use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq)]
pub struct VirtualDom<'a>(Vec<VirtualNode<'a>>);

impl<'a, T: ToString> From<T> for VirtualDom<'a> {
    fn from(s: T) -> VirtualDom<'a> {
        VirtualDom(vec![VirtualNode::Text(s.to_string())])
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum VirtualNode<'a> {
    Text(String),
    Element(VirtualElement<'a>),
}

#[derive(Debug,PartialEq,Eq)]
pub struct VirtualElement<'a> {
    name: &'a str,
    id: Option<&'a str>,
    class: Vec<&'a str>,
    attributes: HashMap<&'a str, &'a str>,
    child_nodes: Vec<VirtualNode<'a>>,
}

impl<'a> VirtualElement<'a> {
    fn new() -> Self {
        VirtualElement {
            name: "",
            id: None,
            class: Vec::new(),
            attributes: HashMap::new(),
            child_nodes: Vec::new(),
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
    () => (|_: &mut::VirtualElement| vec![]);
    ([$($key:ident=$val:expr)*]$($inner:tt)*) => (|el: &mut::VirtualElement| {
        $(el.attributes.insert(stringify!($key), $val);)*
        inner_template!($($inner)*)(&mut el)
    });
    (>($($inner_parens:tt)*)$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_parens);
        el.child_nodes.push(::VirtualNode::Element(el_parens));

        let mut el_remaining = ::VirtualElement::new();
        inner_template!($($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));

        Vec::<::VirtualNode>::new()
    });
    (>$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let el_remaining_additional = inner_template!($($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));

        vec![el_remaining_additional]
    });
    (+($($inner_parens:tt)*)$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        let mut el_parens_additional = inner_template!($($inner)*)(&mut el_parens);

        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional = inner_template!($($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(el_parens);
        els.append(&mut el_parens_additional);

        els.push(el_remaining);
        els.append(&mut el_remaining_additional);
        els
    });
    (+$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional = inner_template!($($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(el_remaining);
        els.append(&mut el_remaining_additional);
        els
    });
    (.$class:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.class.push(stringify!($class));
        inner_template!($($inner)*)(el)
    });
    (#$id:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.id = Some(stringify!($id));
        inner_template!($($inner)*)(el)
    });
    ($name:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.name = stringify!($name);
        inner_template!($($inner)*)(el)
    });
    ($bind:block$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.child_nodes.nodes().append(&mut ::VirtualDom::from($bind).nodes());
        inner_template!($($inner)*)(&mut el)
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn template_macro() {
        let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        let t1 = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        assert_eq!(t, t1);
    }
}
