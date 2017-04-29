use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq)]
pub struct VirtualDom<'a>(Vec<VirtualNode<'a>>);

impl<'a, T: ToString> From<T> for VirtualDom<'a> {
    fn from(s: T) -> VirtualDom<'a> {
        VirtualDom(vec![VirtualNode::Text(s.to_string())])
    }
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum VirtualNode<'a> {
    Text(String),
    Element(VirtualElement<'a>),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VirtualElement<'a> {
    name: &'a str,
    attributes: HashMap<&'a str, &'a str>,
    child_nodes: Vec<VirtualNode<'a>>,
}

impl<'a> VirtualElement<'a> {
    fn new() -> Self {
        VirtualElement {
            name: "",
            attributes: HashMap::new(),
            child_nodes: Vec::new(),
        }
    }
}

macro_rules! template {
    ($($inner:tt)*) => ({
        let mut el = ::VirtualElement::new();
        // "+" is disallowed at the top level, so no additional elements will be returned
        let _ = inner_template!(top_level, $($inner)*)(&mut el);
        el
    });
}

macro_rules! inner_template {
    ($tl:ident, ) => (|_: &mut::VirtualElement| vec![]);
    (not_top_level, [$($key:ident=$val:expr)*]$($inner:tt)*) => (|el: &mut::VirtualElement| {
        $(el.attributes.insert(stringify!($key), $val);)*
        inner_template!(not_top_level, $($inner)*)(&mut el)
    });
    ($tl:ident, >($($inner_parens:tt)*)$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        let el_parens_additional = inner_template!(not_top_level, $($inner)*)(&mut el_parens);
        el.child_nodes.push(::VirtualNode::Element(el_parens));
        for additional in el_parens_additional {
            el.child_nodes.push(::VirtualNode::Element(additional));
        }

        let mut el_remaining = ::VirtualElement::new();
        let el_remaining_additional = inner_template!(not_top_level, $($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));
        for additional in el_remaining_additional {
            el.child_nodes.push(::VirtualNode::Element(additional));
        }

        Vec::<::VirtualNode>::new()
    });
    ($tl:ident, >$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let el_remaining_additional = inner_template!(not_top_level, $($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));

        vec![el_remaining_additional]
    });
    (not_top_level, +($($inner_parens:tt)*)$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        let mut el_parens_additional = inner_template!(not_top_level, $($inner)*)(&mut el_parens);

        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional =
            inner_template!(not_top_level, $($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(el_parens);
        els.append(&mut el_parens_additional);

        els.push(el_remaining);
        els.append(&mut el_remaining_additional);
        els
    });
    (not_top_level, +$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional =
            inner_template!(not_top_level, $($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(el_remaining);
        els.append(&mut el_remaining_additional);
        els
    });
    ($tl:ident, .$classes:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let classes = if let Some(existing_classes) = el.attributes.get("class") {
            &(existing_classes.to_string() + " " + stringify!($classes))
        } else {
            stringify!($classes)
        };
        el.attributes.insert("class", classes);
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, #$id:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.attributes.insert("id", stringify!($id));
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, $name:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.name = stringify!($name);
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, $bind:block$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.child_nodes.nodes().append(&mut ::VirtualDom::from($bind).nodes());
        inner_template!($tl, $($inner)*)(&mut el)
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn template_macro() {
        let a = template!(#asdf);
        let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        let t1 =
            template!(.video>.sidebar>(.asdf#a+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        assert_eq!(t, t1);
    }
}
