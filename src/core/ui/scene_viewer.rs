use std::path::PathBuf;
use macroquad::color::{Color, BLACK};
use macroquad::prelude::{draw_rectangle_ex, draw_rectangle_lines_ex, DrawRectangleParams, Font};
use crate::constants::{PANEL_BG_FILL, PANEL_OUTLINE_FILL, PANEL_OUTLINE_THICKNESS};
use crate::core::components::scene::Scene;
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;
use crate::core::ui::button::Button;

pub struct SceneViewer<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,

    pub fill: Color,
    pub outline: Color,

    pub scenes: Vec<Scene<'a>>,
    pub scene_view: Vec<Button<'a>>,
    pub selected_scene: usize,
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

    fn set_x(&mut self, new: f32) {
        todo!()
    }

    fn set_y(&mut self, new: f32) {
        todo!()
    }
}

impl MouseInteract for SceneViewer<'_> {}

impl Object for SceneViewer<'_> {
    fn get_icon(&self) -> Option<PathBuf> {
        None
    }

    fn get_thickness(&self) -> f32 {
        todo!()
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

    fn draw(&mut self) {
        draw_rectangle_ex(
            self.x, self.y,
            self.width, self.height,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: self.fill,
            }
        );
        draw_rectangle_lines_ex(
            self.x, self.y,
            self.width, self.height,
            self.thickness,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: self.outline,
            }
        );
    }
}

impl SceneViewer<'_> {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        SceneViewer {
            x, y,
            width, height,

            thickness: PANEL_OUTLINE_THICKNESS,
            fill: PANEL_BG_FILL,
            outline: PANEL_OUTLINE_FILL,
            scenes: vec![],
            scene_view: vec![],
            selected_scene: 0,
        }
    }
}