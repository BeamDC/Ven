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

pub const PANEL_BG_FILL: Color = normalize_color!((45, 45, 45));
pub const PANEL_OUTLINE_THICKNESS: f32 = 1f32;
pub const PANEL_OUTLINE_FILL: Color = normalize_color!((30, 30, 30));

pub const TOOLBAR_BUTTON_FILL: Color = normalize_color!((58, 58, 58));
pub const TOOLBAR_BUTTON_HIGHLIGHT: Color = normalize_color!((86, 86, 86));
pub const TOOLBAR_HEIGHT: f32 = 60f32;
pub const TOOLBAR_BUTTON_SIZE: f32 = 50f32;

pub const SCENE_VIEWER_WIDTH: f32 = 200f32;

pub const PLAYER_NODE_OUTLINE: Color = normalize_color!((158, 66, 245));
pub const NPC_NODE_OUTLINE: Color = normalize_color!((27, 119, 224));
pub const STORY_NODE_OUTLINE: Color = normalize_color!((110, 110, 110));

pub const NODE_MANAGER_FILL: Color = normalize_color!((15, 15, 15));
pub const NODE_MANAGER_LINES: Color = normalize_color!((30, 30, 30));

pub const NODE_SELECT_OUTLINE: Color = normalize_color!((255, 175, 15));
pub const NODE_HIGHLIGHT: Color = normalize_color!((70, 70, 70));
pub const NODE_TILE_WIDTH: f32 = 200f32;
pub const NODE_TILE_HEIGHT: f32 = 125f32;

pub const DOUBLE_CLICK_DELAY_MS: u128 = 300u128;

// egui related stuff
// pub const DEFAULT_WINDOW_POS: (f32, f32) = (SCENE_VIEWER_WIDTH, TOOLBAR_HEIGHT);
pub const DEFAULT_WINDOW_POS: (f32, f32) = (SCENE_VIEWER_WIDTH * 1.5, TOOLBAR_HEIGHT * 1.5);