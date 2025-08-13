use crate::core::traits::interaction::MouseInteract;
use crate::core::traits::object::Object;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};

// Drag state to track dragging behavior
#[derive(Debug, Clone)]
pub struct Drag {
    pub is_dragging: bool,
    pub drag_offset: (f32, f32),
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_offset: (0.0, 0.0),
        }
    }
}

pub trait Draggable: Object + MouseInteract + Clone {
    fn get_drag(&self) -> &Drag;
    fn get_drag_state_mut(&mut self) -> &mut Drag;

    fn start_drag(&mut self, mouse_x: f32, mouse_y: f32) {
        let offset = (
            self.get_x() - mouse_x,
            self.get_y() - mouse_y,
        );
        let drag_state = self.get_drag_state_mut();
        drag_state.is_dragging = true;
        drag_state.drag_offset = offset;
    }

    fn stop_drag(&mut self) {
        self.get_drag_state_mut().is_dragging = false;
    }

    fn update_drag(&mut self) -> bool{
        let (mouse_x, mouse_y) = mouse_position();
        let drag = self.get_drag();

        // Check if we should start dragging
        if !drag.is_dragging {
            if self.is_pressed(MouseButton::Left) {
                self.start_drag(mouse_x, mouse_y);
                return true
            }
            false
        }

        // If we're currently dragging
        else {
            if is_mouse_button_down(MouseButton::Left) {
                // Update position based on mouse position and offset
                let drag_state = self.get_drag();
                let new_x = mouse_x + drag_state.drag_offset.0;
                let new_y = mouse_y + drag_state.drag_offset.1;

                self.set_x(new_x);
                self.set_y(new_y);
            } else {
                // Mouse button released, stop dragging
                self.stop_drag();
            }
            true
        }
    }
}