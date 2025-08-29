use std::path::PathBuf;
use macroquad::color::{Color, BLACK, RED, WHITE};
use macroquad::input::mouse_position;
use macroquad::prelude::{draw_circle, draw_circle_lines, Font};
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;

#[derive(Clone)]
pub struct Connection {
    pub x: f32,
    pub y: f32,
    pub thickness: f32,
    pub radius: f32,
}

impl Pos for Connection {
    fn get_width(&self) -> f32 {
        self.radius
    }

    fn get_height(&self) -> f32 {
        self.radius
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, new: f32) {
        self.x = new;
    }

    fn set_y(&mut self, new: f32) {
        self.y = new;
    }
}

impl MouseInteract for Connection {
    fn is_hovered(&self) -> bool {
        let (x, y) = mouse_position();
        let d = (
            (x - self.x) * (x - self.x) +
            (y - self.y) * (y - self.y)
        ).sqrt();
        d <= self.radius
    }
}

impl Object for Connection {
    fn get_icon(&self) -> Option<PathBuf> {
        todo!()
    }

    fn get_thickness(&self) -> f32 {
        todo!()
    }

    fn get_text(&self) -> String {
        todo!()
    }

    fn get_font(&self) -> Option<&Font> {
        todo!()
    }

    fn get_font_size(&self) -> u16 {
        todo!()
    }

    fn get_text_colour(&self) -> Color {
        todo!()
    }

    fn draw(&mut self) {
        let color = match self.is_hovered() {
            true => RED,
            false => WHITE,
        };
        draw_circle(self.get_x(), self.get_y(), self.radius, color);
        draw_circle_lines(self.get_x(), self.get_y(), self.radius, self.thickness, BLACK);
    }
}

impl Connection {
    pub fn new(x: f32, y: f32) -> Connection {
        Connection {
            x,
            y,
            thickness: 0.0,
            radius: 5.0,
        }
    }
}