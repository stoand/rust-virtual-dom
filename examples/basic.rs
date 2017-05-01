extern crate stdweb;
#[macro_use]
extern crate virtual_dom;

#[cfg(target_os="emscripten")]
fn main() {
    stdweb::initialize();
    virtual_dom::render::render_virtual_dom("#vdom-root".to_string(), template!(div>input[type="button" value="button 1"]));
}

#[cfg(not(target_os="emscripten"))]
fn main() {}