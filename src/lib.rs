use std::collections::HashMap;

pub enum VirtualElementConfig {
    Nm(String),
    At(String, String),
    Ev(String, String),
}

pub struct VirtualElement {
    /// defaults to "div"
    name: Option<String>,
    attrs: HashMap<String, String>,
    events: Vec<(String, String)>,
    children: Vec<VirtualElement>,
}

impl VirtualElement {
    fn new() -> VirtualElement {
        VirtualElement {
            name: None,
            attrs: HashMap::new(),
            events: Vec::new(),
            children: Vec::new(),
        }
    }
}

pub fn el(partial_elems: &mut [VirtualElement]) -> VirtualElement {
    let mut elem = VirtualElement {
        name: None,
        attrs: HashMap::new(),
        events: Vec::new(),
        children: Vec::new(),
    };

    for partial_elem in partial_elems {
        if let Some(_) = partial_elem.name {
            elem.name = partial_elem.name.clone();
        }

        for (k, v) in partial_elem.attrs.drain() {
           elem.attrs.insert(k, v); 
        }

        elem.events.extend(partial_elem.events.drain(..));
        elem.children.extend(partial_elem.children.drain(..));
    }

    elem
}

impl From<VirtualElementConfig> for VirtualElement {
    fn from(config: VirtualElementConfig) -> VirtualElement {
        use VirtualElementConfig::*;

        match config {
            Ev(name, action) => VirtualElement { events: vec![(name, action)], ..VirtualElement::new() },
            _ => VirtualElement::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
