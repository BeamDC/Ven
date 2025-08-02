use macroquad::input::{is_mouse_button_down, MouseButton};

// todo : make this require a size & pos trait,
//   so we can just define stuff as default
pub trait MouseInteract {
    /// if the object is being hovered over by the mouse
    fn is_hovered(&self) -> bool;

    /// if a given mouse button is clicked when the object is hovered
    fn is_pressed(&self, mouse_button: MouseButton) -> bool {
        self.is_hovered() && is_mouse_button_down(mouse_button)
    }

    // todo : the on_press functions may need some work,
    //  this is just a prototype
    /// run a function when the object is pressed
    fn on_press<F>(&self, mouse_button: MouseButton, f: F)
    where
        F: FnOnce()
    {
        if self.is_pressed(mouse_button) {
            f()
        }
    }

    /// run a function which can mutate the object when it is pressed
    fn on_press_mut<F>(&self, mouse_button: MouseButton, mut f: F)
    where
        F: FnMut()
    {
        if self.is_pressed(mouse_button) {
            f()
        }
    }
}