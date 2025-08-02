use crate::core::ui::interaction::{MouseInteract, Pos};
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams, draw_rectangle_lines_ex};
use macroquad::text::{draw_text_ex, measure_text, TextParams};

/// the standard clickable button that will be used in both editor and game ui
pub struct Button {
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
}

impl MouseInteract for Button {}
impl Pos for Button {
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

impl Default for Button {
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
        }
    }
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, thickness: f32, label: String) -> Button {
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
            None,
            25,
            1.0,
        );
    }
}