macro_rules! template {
    () => (1);
    (($($inner:tt)*)) => (1);
    (.$class:ident$($inner:tt)*) => (1);
    ($name:ident.$class:ident$($inner:tt)*) => (1);

    (+$($inner:tt)*) => (template!($ds, $($inner)*));
    (>$bind:block) => (1);
    (>$($inner:tt)*) => (1);
    ($bind:block) => (1);
    ([$( $key:ident=$val:expr)*]) => (1);
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
        let some = 1;

        let t = template!(.video>div.sidebar>.asdf+a.a[href="asdf" type="asdf"]>{some});
        assert_eq!(t, 1);
    }
}
