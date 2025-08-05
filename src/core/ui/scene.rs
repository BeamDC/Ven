use crate::core::ui::button::Button;
use crate::core::ui::layer::Layer;

pub struct Scene<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub layers: Vec<Layer>,
    pub layer_view: Vec<Button<'a>>,
}