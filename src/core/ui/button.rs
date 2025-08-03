use crate::core::ui::interaction::{MouseInteract, Pos};
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams, draw_rectangle_lines_ex};
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};

/// the standard clickable button that will be used in both editor and game ui
pub struct Button<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub label: String,

    // style
    pub stroke: Color,
    pub fill: Color,
    pub text_colour: Color,
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

            stroke: Color::from_rgba(0,0,0,0),
            fill: Color::from_rgba(0,0,0,0),
            text_colour: Color::from_rgba(0,0,0,0),
            text_size: 8,
            font: None,
        }
    }
}

impl Button<'static> {
    pub fn new(
        x: f32, y: f32, width: f32, height: f32, thickness: f32, label: String
    ) -> Button<'static> {
        Button {
            x,
            y,
            width,
            height,
            thickness,
            label,
            ..Self::default()
        }
    }

    pub fn draw(&self) {
        let fill = self.fill;
        let stroke = self.stroke;

        // todo : resize if text is too large for button

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

        // draw text
        // todo : center text on the button
        let dim = measure_text(
            self.label.as_str(),
            self.font,
            self.text_size,
            1.0,
        );

        let pad_x = self.x + (self.width - dim.width) / 2.0;
        let pad_y = self.y + (self.height + dim.height) / 2.0;

        draw_text_ex(
            self.label.as_str(),
            pad_x,
            pad_y,
            TextParams {
                font: self.font,
                font_size: self.text_size,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: self.text_colour,
            }
        );
    }
}