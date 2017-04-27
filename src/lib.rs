

macro_rules! template {
    ($data_struct:expr, .$sel:ident) => ("<div class=\"".to_string() + stringify!($sel) +  "\"></div>");
    ($data_struct:expr, .$sel:ident>$(desc:tt)*) => ("<$sel>".to_string() + template!($data_struct, .asdf) + "</$sel>");
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
        // let t = template!(User, .video>(.h2>User(user))+img[src=@url alt=@description]);
        // assert_eq!(template!(User, .video), "<div class=\"video\"></div>");
        assert_eq!(template!(User, .video>.h2), "<div class=\"video\"></div>");
    }
}
