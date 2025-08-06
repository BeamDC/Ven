use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::ImageFormat;
use macroquad::shapes::{draw_rectangle_ex, draw_rectangle_lines_ex, DrawRectangleParams};
use macroquad::text::Font;
use std::path::PathBuf;
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;

/// the standard clickable button that will be used in both editor and game ui
pub struct Button<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub label: String,
    pub icon: PathBuf,
    pub icon_format: Option<ImageFormat>,

    // style
    pub stroke: Color,
    pub fill: Color,
    pub font_size: Color,
    pub text_size: u16,
    pub font: Option<&'a Font>
}

impl MouseInteract for Button<'_> {}
impl Pos for Button<'_> {
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

impl Object for Button<'_> {
    fn get_border_thickness(&self) -> f32 {
        self.thickness
    }

    fn get_icon(&self) -> Option<PathBuf> {
        Some(self.icon.clone())
    }

    fn get_text(&self) -> String {
        self.label.clone()
    }

    fn get_font(&self) -> Option<&Font> {
        self.font
    }

    fn get_font_size(&self) -> u16 {
        self.text_size
    }

    fn get_text_colour(&self) -> Color {
        self.font_size
    }

    fn draw(&self) {
        let fill = self.fill;
        let stroke = self.stroke;

        // draw base
        draw_rectangle_ex(
            self.x,
            self.y,
            self.width,
            self.height,
            DrawRectangleParams {
                offset: Vec2::new(0.0, 0.0),
                rotation: 0.0,
                color: fill
            }
        );

        // draw outline
        draw_rectangle_lines_ex(
            self.x,
            self.y,
            self.width,
            self.height,
            self.thickness,
            DrawRectangleParams {
                offset: Vec2::new(0.0, 0.0),
                rotation: 0.0,
                color: stroke
            }
        );

        self.draw_icon(self.icon_format);
        self.draw_text();
    }
}

impl Default for Button<'_> {
    fn default() -> Self {
        Button {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            thickness: 0.0,
            label: String::new(),
            icon: PathBuf::new(),
            icon_format: None,

            stroke: Color::from_rgba(0,0,0,0),
            fill: Color::from_rgba(0,0,0,0),
            font_size: Color::from_rgba(0, 0, 0, 0),
            text_size: 8,
            font: None,
        }
    }
}

impl Button<'static> {
    pub fn new(
        x: f32, y: f32, width: f32, height: f32,
        thickness: f32, label: String,
        icon: Option<PathBuf>, icon_format: Option<ImageFormat>,
    ) -> Button<'static> {
        let icon = icon.unwrap_or(PathBuf::new());
        Button {
            x,
            y,
            width,
            height,
            thickness,
            label,
            icon,
            icon_format,
            ..Self::default()
        }
    }
}