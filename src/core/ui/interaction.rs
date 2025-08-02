use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position, MouseButton};

/// trait for all ui items to relate useful data
pub trait Pos { // todo : better name :P
    /// return the width of the object
    fn get_width(&self) -> f32;

    /// return the height of the object
    fn get_height(&self) -> f32;

    /// return the x coordinate of the top left of the object
    fn get_x(&self) -> f32;

    /// return the y coordinate of the top left of the object
    fn get_y(&self) -> f32;

    /// return the x and y coordinate of the top left of the object
    /// in the form `(x, y)`
    fn get_pos(&self) -> (f32, f32) {
        (self.get_x(), self.get_y())
    }

    /// return the width and height of the object in the form `(width, height)`
    fn get_dimensions(&self) -> (f32, f32) {
        (self.get_width(), self.get_height())
    }
}


pub trait MouseInteract: Pos {
    /// if the object is being hovered over by the mouse
    fn is_hovered(&self) -> bool {
        let (x, y) = self.get_pos();
        let (width, height) = self.get_dimensions();
        let (mx, my) = mouse_position();
        mx >= x && mx <= x + width && my >= y && my <= y + height
    }

    /// if a given mouse button is clicked when the object is hovered
    fn is_pressed(&self, mouse_button: MouseButton) -> bool {
        self.is_hovered() && is_mouse_button_pressed(mouse_button)
    }

    fn on_hover<F>(&self, f: F)
    where
        F: FnOnce()
    {
        if self.is_hovered() {
            f()
        }
    }

    fn on_hover_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self)
    {
        if self.is_hovered() {
            f(self)
        }
    }

    /// run a function when the object is pressed
    fn on_press<F>(&self, mouse_button: MouseButton, f: F)
    where
        F: FnOnce()
    {
        if self.is_pressed(mouse_button) {
            f()
        }
    }

    /// run a function which mutates the object when it is pressed
    fn on_press_mut<F>(&mut self, mouse_button: MouseButton, mut f: F)
    where
        F: FnMut(&mut Self)
    {
        if self.is_pressed(mouse_button) {
            f(self)
        }
    }
}