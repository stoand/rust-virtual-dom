macro_rules! template_props {
    ($key:ident=@$ds_val:ident) => (1);
    ($key:ident=$val:expr) => (1);
}

macro_rules! template {

    ($ds:expr, [$( $key:ident=$val:expr)*]) => ();
    ($ds:expr, .$sel:ident) => (1);

    ($ds:expr, .$sel:ident>$($inner:tt)*) => (template!($ds, $($inner)*));
    ($ds:expr, .$sel:ident+$($inner:tt)*) => (template!($ds, $($inner)*));

    ($ds:expr, +$($inner:tt)*) => (template!($ds, $($inner)*));
    ($ds:expr, >$($inner:tt)*) => (1);


    ($ds:expr, $name:ident.$sel:ident) => (1);
    ($ds:expr, $name:ident.$sel:ident>$($inner:tt)*) => (template!($ds, $($inner)*));

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
        let a = super::User { name: "".into(), views: 3 };

        let t = template!(User, .video>div.sidebar>.asdf+a.a);
        assert_eq!(t, 1);
    }
}
