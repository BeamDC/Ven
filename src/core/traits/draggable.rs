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
    fn get_drag_state(&self) -> &Drag;
    fn get_drag_state_mut(&mut self) -> &mut Drag;

    fn start_drag(&mut self, mouse_x: f32, mouse_y: f32) {
        println!("Drag start");
        let offset = (
            self.get_x() - mouse_x,
            self.get_y() - mouse_y,
        );
        let drag_state = self.get_drag_state_mut();
        drag_state.is_dragging = true;
        drag_state.drag_offset = offset;
    }

    fn stop_drag(&mut self) {
        println!("Drag stopped");
        self.get_drag_state_mut().is_dragging = false;
    }

    fn update_drag(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        let drag_state = self.get_drag_state();

        // Check if we should start dragging
        if !drag_state.is_dragging {
            if self.is_pressed(MouseButton::Left) {
                self.start_drag(mouse_x, mouse_y);
            }
        }

        // If we're currently dragging
        else {
            if is_mouse_button_down(MouseButton::Left) {
                // Update position based on mouse position and offset
                let drag_state = self.get_drag_state();
                let new_x = mouse_x + drag_state.drag_offset.0;
                let new_y = mouse_y + drag_state.drag_offset.1;

                self.set_x(new_x);
                self.set_y(new_y);
            } else {
                // Mouse button released, stop dragging
                self.stop_drag();
            }
        }
    }

    // Convenience method that can be called in your main loop
    fn handle_drag(&mut self) {
        self.update_drag();
    }
}