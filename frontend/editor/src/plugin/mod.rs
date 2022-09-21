use web_sys::Element;


/// Trait impl for plugins
pub trait Plugin {
    /// handle plugin for each element
    /// note this is the rendered element i.e. this is run after EditorElement 
    /// is converted into element 
    fn handle(&mut self, e: Element);
}
