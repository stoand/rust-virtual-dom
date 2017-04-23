pub enum VirtualDom<'a> {
    /// class
    Cl(&'a str),
    // id
    Id(&'a str),
    /// attribute
    At(&'a str, &'a str),
    /// on event
    On(&'a str, &'a str),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::VirtualDom::*;

        let dom = ("div", Id("one"), Cl("asdf"), At("name", "asdf"),
            ("div", Id("two")),
            ("div", Id("three")),
            ("div", Id("three"),
                ("a", Id("four"), Cl("a"), At("", "")),
                ("a", Id("five")),
            ),
        );
    }
}
