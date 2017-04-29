use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VirtualDom(Vec<VirtualNode>);

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
    name: String,
    attributes: HashMap<String, String>,
    child_nodes: Vec<VirtualNode>,
}

impl VirtualElement {
    fn new() -> Self {
        VirtualElement {
            name: "div".to_string(),
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
    ($tl:ident, ) => (|_: &mut::VirtualElement| Vec::<::VirtualNode>::new());
    (not_top_level, [$($key:ident=$val:expr)*]$($inner:tt)*) => (|el: &mut::VirtualElement| {
        $(el.attributes.insert(stringify!($key).to_string(), $val.to_string());)*
        inner_template!(not_top_level, $($inner)*)(&mut el)
    });
    ($tl:ident, >($($inner_parens:tt)*)$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        let mut el_parens_additional = inner_template!(not_top_level, $($inner)*)(&mut el_parens);
        el.child_nodes.push(::VirtualNode::Element(el_parens));
        el.child_nodes.append(&mut el_parens_additional);

        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional = inner_template!(not_top_level, $($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));
        el.child_nodes.append(&mut el_remaining_additional);

        Vec::<::VirtualNode>::new()
    });
    ($tl:ident, >$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional = inner_template!(not_top_level, $($inner)*)(&mut el_remaining);
        el.child_nodes.push(::VirtualNode::Element(el_remaining));
        el.child_nodes.append(&mut el_remaining_additional);

        Vec::<::VirtualNode>::new()
    });
    (not_top_level, +($($inner_parens:tt)*)$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_parens = ::VirtualElement::new();
        let mut el_parens_additional = inner_template!(not_top_level, $($inner)*)(&mut el_parens);

        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional =
            inner_template!(not_top_level, $($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(::VirtualNode::Element(el_parens));
        els.append(&mut el_parens_additional);

        els.push(::VirtualNode::Element(el_remaining));
        els.append(&mut el_remaining_additional);
        els
    });
    (not_top_level, +$($inner:tt)*) => (|_: &mut::VirtualElement| {
        let mut el_remaining = ::VirtualElement::new();
        let mut el_remaining_additional =
            inner_template!(not_top_level, $($inner)*)(&mut el_remaining);

        let mut els = Vec::new();

        els.push(::VirtualNode::Element(el_remaining));
        els.append(&mut el_remaining_additional);
        els
    });
    ($tl:ident, {$bind:expr}$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.child_nodes.append(&mut ::VirtualDom::from($bind).0);
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, .$classes:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        let classes = if let Some(existing_classes) = el.attributes.get("class") {
            existing_classes.to_string() + " " + stringify!($classes)
        } else {
            stringify!($classes).to_string()
        };
        el.attributes.insert("class".to_string(), classes);
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, #$id:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.attributes.insert("id".to_string(), stringify!($id).to_string());
        inner_template!($tl, $($inner)*)(el)
    });
    ($tl:ident, $name:ident$($inner:tt)*) => (|el: &mut::VirtualElement| {
        el.name = stringify!($name).to_string();
        inner_template!($tl, $($inner)*)(el)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_name_class_id() {
        let mut el = VirtualElement::new();
        assert_eq!(el, template!(div));

        el.name = "a".to_string();
        assert_eq!(el, template!(a));

        el.attributes.insert("class".into(), "active red".into());
        assert_eq!(el, template!(a.active.red));

        el.attributes.insert("id".into(), "main".into());
        assert_eq!(el, template!(a#main.active.red));
    }

    #[test]
    fn template_binding() {
        let mut el = VirtualElement::new();
        el.child_nodes.push(VirtualNode::Text("some inner text".into()));
        assert_eq!(el, template!(div{"some inner text"}));
    }


        // let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        // let t1 =
        //     template!(.video>.sidebar>(.asdf#a+.a[href="asdf" type="asdf"]@"inner text")+.a+(.b));
        // assert_eq!(t, t1);
}
