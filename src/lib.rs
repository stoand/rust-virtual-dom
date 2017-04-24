#[derive(Debug, Eq, PartialEq)]
pub enum VirtualDom<'a> {
    // name, ie "div"
    Nm(&'a str),
    /// class
    Cl(&'a str),
    ClOp(Option<&'a str>),
    // id
    Id(&'a str),
    IdOp(Option<&'a str>),
    /// attribute
    At(&'a str, &'a str),
    AtOp(Option<(&'a str, &'a str)>),
    /// on event
    On(&'a str, &'a str),
    /// additional configuration with an dynamic length
    Cfg(Vec<VirtualDom<'a>>),
    /// configuration for a child element
    Child(Vec<VirtualDom<'a>>),
    /// a text-only element
    /// note: this cannot be used alongside other config
    Text(&'a str),
}

///
macro_rules! vdom {
    (($name:expr, ($( $prop:expr ),*), ( $( $child:tt ),* ) )) => {
        {
            use ::VirtualDom::*;
            let mut elem: Vec<::VirtualDom> = Vec::new();
            elem.push(Nm($name));
            $(
                elem.push($prop);
            )*
            $(
                let a = $child;
                elem.push(Child(vdom!($child)));
            )*
            elem
        }
    };
    (($name:expr, $props:tt )) => {
        {
            vdom!(($name, $props, ()))
        }
    };
    ($text:expr) => {
        {
            vec![::VirtualDom::Text($text)]
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn vdom_macro_works() {
        use super::VirtualDom::*;

        let generated_dom = vdom!(
            ("div", (Id("root"), ClOp(Some("some_class"))), (
                "some random text",
                ("a", (At("href", "https://google.com")))
            ))
        );

        let expected_dom = vec![
            Nm("div"),
            Id("root"),
            ClOp(Some("some_class")),
            Child(vec![Text("some random text")]),
            Child(vec![Nm("a"), At("href", "https://google.com")]),
        ];

        assert_eq!(generated_dom, expected_dom);
    }
}
