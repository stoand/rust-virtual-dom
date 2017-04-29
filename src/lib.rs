macro_rules! template {
    (@$bind:expr) => (1);
    (>($($inner:tt)*)) => (template!($($inner)*));
    (>$($inner:tt)*) => (template!($($inner)*));
    (+$($inner:tt)*) => (template!($($inner)*));
    ([$( $key:ident=$val:expr)*]$($inner:tt)*) => (template!($($inner)*));
    (.$class:ident$($inner:tt)*) => (template!($($inner)*));
    ($name:ident.$class:ident$($inner:tt)*) => (template!($($inner)*));
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

        let t = template!(.video>.sidebar>(.asdf+.a[href="asdf" type="asdf"]@some));
        assert_eq!(t, 1);
    }
}
