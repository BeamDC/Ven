use macroquad::color::Color;

/// macro that just turns rgb values into normalized color values
/// to allow us to have colors as constants
macro_rules! normalize_color { // rgb values are more readable
    ($colors: expr) => {
        Color::new(
            $colors.0 as f32 / 255.0,
            $colors.1 as f32 / 255.0,
            $colors.2 as f32 / 255.0,
            1.0,
        )
    };
}

pub const TOOLBAR_BG_FILL: Color = normalize_color!((45, 45, 45));
pub const TOOLBAR_BUTTON_FILL: Color = normalize_color!((58, 58, 58));
pub const TOOLBAR_BUTTON_HIGHLIGHT: Color = normalize_color!((86, 86, 86));
pub const TOOLBAR_HEIGHT: f32 = 60f32;