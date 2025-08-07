use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_text_ex, draw_texture_ex, measure_text, DrawTextureParams, Image, ImageFormat, TextParams};
use macroquad::text::Font;
use macroquad::texture::Texture2D;
use std::path::PathBuf;
use crate::core::traits::interaction::Pos;

/// The standard UI Object, assumed to be a rectangle by default
pub trait Object: Pos {
    fn get_icon(&self) -> Option<PathBuf>;

    // text related functions
    fn get_text(&self) -> String;
    fn get_font(&self) -> Option<&Font>;
    fn get_font_size(&self) -> u16;
    fn get_text_colour(&self) -> Color;
    /// draw the texture associated with an [`Object`],
    /// justified in the center of the object.
    fn draw_icon(&self, format: Option<ImageFormat>) {
        let icon = self.get_icon();
        if icon.is_none() { return }

        let file_path = icon.unwrap();
        if file_path.to_str().unwrap_or("").is_empty() { return }
        let bytes = match std::fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file_path, e);
                return;
            }
        };

        let width = (4.0 * self.get_width() / 5.0) as u16;
        let height = (4.0 * self.get_height() / 5.0) as u16;

        let image = match Image::from_file_with_format(&*bytes, format) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("{}", e);
                return
            },
        };
        let texture = Texture2D::from_image(&image);
        let pad_x = self.get_x() + (width / 8) as f32;
        let pad_y = self.get_y() + (height / 8) as f32;

        draw_texture_ex(
            &texture,
            pad_x, pad_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(width as f32, height as f32)),
                ..Default::default()
            }
        );
    }
    /// draw the text associated with an [`Object`],
    /// justified in the center of the object.
    fn draw_text(&self) {
        let mut dim = measure_text(
            self.get_text().as_str(),
            self.get_font(),
            self.get_font_size(),
            1.0,
        );

        let mut font_size = self.get_font_size();
        while dim.width >= self.get_width() || dim.height >= self.get_height() {
            font_size -= 1;
            dim = measure_text(
                self.get_text().as_str(),
                self.get_font(),
                font_size,
                1.0,
            );
        }

        let pad_x = self.get_x() + (self.get_width() - dim.width) / 2.0;
        let pad_y = self.get_y() + (self.get_height() + dim.height) / 2.0;

        let params = TextParams {
            font: self.get_font(),
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: self.get_text_colour(),
        };

        draw_text_ex(
            self.get_text().as_str(),
            pad_x,
            pad_y,
            params
        );
    }

    fn draw(&self);
}