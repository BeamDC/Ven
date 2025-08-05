use crate::core::ui::interaction::{MouseInteract, Pos};
use crate::core::ui::object::Object;
use macroquad::color::{Color, BLACK};
use macroquad::prelude::{Font, Texture2D};
use std::path::PathBuf;

pub struct Layer {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub path: PathBuf,
    pub texture: Texture2D,
    // todo : preview will be used as the button icon in scenes
    // pub preview: IDK
    // pub effects: Vec<Effect>
}

impl Pos for Layer {
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

impl Object for Layer {
    fn get_border_thickness(&self) -> f32 {
        0.0
    }

    fn get_icon(&self) -> Option<PathBuf> {
        None // todo : in the future this can be a
    }

    fn get_text(&self) -> String {
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

impl MouseInteract for Layer {}