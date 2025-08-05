use std::path::PathBuf;
use macroquad::color::{Color, BLACK};
use macroquad::input::MouseButton;
use macroquad::prelude::{DrawMode, Font, ImageFormat};
use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};
use crate::constants::{TOOLBAR_BG_FILL, TOOLBAR_BUTTON_FILL, TOOLBAR_BUTTON_HIGHLIGHT};
use crate::core::ui::button::Button;
use crate::core::ui::interaction::{MouseInteract, Pos};
use crate::core::ui::object::Object;

pub struct Toolbar<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub fill: Color,
    
    pub import: Button<'a>,
    pub export: Button<'a>,
}

impl Pos for Toolbar<'_> {
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

impl Object for Toolbar<'_> {
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
        draw_rectangle_ex(
            self.x, self.y,
            self.width, self.height,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: self.fill,
            }
        );
        self.import.draw();
        self.export.draw();
    }
}

impl MouseInteract for Toolbar<'_> {}

impl Toolbar<'_> {
    pub fn new(width: f32, height: f32) -> Toolbar<'static> {
        let size = height - 20.0;
        let mut import = Button::new(
            10.0, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/import_file_128.png")),
            Some(ImageFormat::Png)
        );

        let mut export = Button::new(
            60.0, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/save_as_128.png")),
            Some(ImageFormat::Png)
        );
        import.fill = TOOLBAR_BUTTON_FILL;
        export.fill = TOOLBAR_BUTTON_FILL;

        Toolbar {
            x: 0.0,
            y: 0.0,
            width,
            height,
            fill: TOOLBAR_BG_FILL,
            import,
            export,
        }
    }

    pub fn handle_input(&mut self) {
        // import
        if !self.import.is_hovered() {
            self.import.fill = TOOLBAR_BUTTON_FILL;
        }
        self.import.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        self.import.on_press(MouseButton::Left, || {
            todo!("Open rfd and pick a project file")
        });

        // export
        if !self.export.is_hovered() {
            self.export.fill = TOOLBAR_BUTTON_FILL;
        }
        self.export.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        self.export.on_press(MouseButton::Left, || {
            todo!("Open rfd and pick a location to save the current project")
        });
    }
}