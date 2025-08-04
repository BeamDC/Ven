use std::path::PathBuf;
use macroquad::color::{Color, BLACK};
use macroquad::prelude::Font;
use crate::core::ui::button::Button;
use crate::core::ui::interaction::{MouseInteract, Pos};
use crate::core::ui::object::Object;

pub struct SceneViewer<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub scenes: Vec<Button<'a>>,
}

impl Pos for SceneViewer<'_> {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }
}

impl MouseInteract for SceneViewer<'_> {}

impl Object for SceneViewer<'_> {
    fn get_border_thickness(&self) -> f32 {
        0.0
    }

    fn get_icon(&self) -> Option<PathBuf> {
        None
    }

    fn get_text(&self) -> String {
        // this will not have any text
        String::new()
    }

    fn get_font(&self) -> Option<&Font> {
        None
    }

    fn get_font_size(&self) -> u16 {
        0
    }

    fn get_text_colour(&self) -> Color {
        BLACK
    }

    fn draw(&self) {
        todo!()
    }
}