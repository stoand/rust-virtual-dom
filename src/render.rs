use super::VirtualElement;

pub fn render_virtual_dom(selector: String, virtual_element: VirtualElement) {
   js! {
       document.querySelector(@{selector}).innerHTML = @{virtual_element.to_string()}
   } 
}