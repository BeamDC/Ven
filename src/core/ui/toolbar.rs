use crate::constants::{PANEL_OUTLINE_THICKNESS, PANEL_BG_FILL, TOOLBAR_BUTTON_FILL, TOOLBAR_BUTTON_HIGHLIGHT, PANEL_OUTLINE_FILL, TOOLBAR_BUTTON_SIZE};
use crate::core::ui::button::Button;
use macroquad::color::{Color, BLACK};
use macroquad::input::MouseButton;
use macroquad::prelude::{Font, ImageFormat};
use macroquad::shapes::{draw_rectangle_ex, draw_rectangle_lines_ex, DrawRectangleParams};
use std::path::PathBuf;
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;
use crate::core::ui::toolbar::ToolbarAction::{AddNpc, AddPlayer, AddStory};

pub enum ToolbarAction {
    AddPlayer,
    AddNpc,
    AddStory,
}

pub struct Toolbar<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,

    pub fill: Color,
    pub outline: Color,
    
    pub import: Button<'a>,
    pub export: Button<'a>,
    pub add_player: Button<'a>,
    pub add_npc: Button<'a>,
    pub add_story: Button<'a>,
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

    fn set_x(&mut self, new: f32) {
        todo!()
    }

    fn set_y(&mut self, new: f32) {
        todo!()
    }
}

impl Object for Toolbar<'_> {
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
        self.import.draw();
        self.export.draw();
        self.add_player.draw();
        self.add_npc.draw();
        self.add_story.draw();
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
            10.0 + TOOLBAR_BUTTON_SIZE, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/save_as_128.png")),
            Some(ImageFormat::Png)
        );

        let mut add_player = Button::new(
            10.0 + TOOLBAR_BUTTON_SIZE * 2.0, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/player_128.png")),
            Some(ImageFormat::Png)
        );

        let mut add_npc = Button::new(
            10.0 + TOOLBAR_BUTTON_SIZE * 3.0, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/npc_128.png")),
            Some(ImageFormat::Png)
        );

        let mut add_story = Button::new(
            10.0 + TOOLBAR_BUTTON_SIZE * 4.0, 10.0,
            size, size, 2.0,
            String::new(),
            Some(PathBuf::from("src/assets/icons/story_128.png")),
            Some(ImageFormat::Png)
        );
        
        import.fill = TOOLBAR_BUTTON_FILL;
        export.fill = TOOLBAR_BUTTON_FILL;

        Toolbar {
            x: 0.0,
            y: 0.0,
            width,
            height,
            thickness: PANEL_OUTLINE_THICKNESS,
            fill: PANEL_BG_FILL,
            outline: PANEL_OUTLINE_FILL,
            import,
            export,
            add_player,
            add_npc,
            add_story,
        }
    }

    pub fn handle_input(&mut self) -> Option<ToolbarAction> {
        // import
        self.import.fill = TOOLBAR_BUTTON_FILL;
        self.import.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        self.import.on_press(MouseButton::Left, || {
            todo!("Open rfd and pick a project file")
        });

        // export
        self.export.fill = TOOLBAR_BUTTON_FILL;
        self.export.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        self.export.on_press(MouseButton::Left, || {
            todo!("Open rfd and pick a location to save the current project")
        });

        // add_player
        self.add_player.fill = TOOLBAR_BUTTON_FILL;
        self.add_player.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        let add_player = self.add_player.on_press(MouseButton::Left, || -> Option<ToolbarAction> {
            println!("Player Added!");
            Some(AddPlayer)
        }).unwrap_or(None);

        // add_npc
        self.add_npc.fill = TOOLBAR_BUTTON_FILL;
        self.add_npc.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        let add_npc = self.add_npc.on_press(MouseButton::Left, || -> Option<ToolbarAction> {
            println!("NPC Added!");
            Some(AddNpc)
        }).unwrap_or(None);

        // add_story
        self.add_story.fill = TOOLBAR_BUTTON_FILL;
        self.add_story.on_hover_mut(|b| {
            b.fill = TOOLBAR_BUTTON_HIGHLIGHT;
        });
        let add_story = self.add_story.on_press(MouseButton::Left, || -> Option<ToolbarAction> {
            println!("Story Added!");
            Some(AddStory)
        }).unwrap_or(None);

        let actions = (add_player, add_npc, add_story);
        match actions {
            (Some(AddPlayer), _, _) => Some(AddPlayer),
            (_, Some(AddNpc), _) => Some(AddNpc),
            (_, _, Some(AddStory)) => Some(AddStory),
            _ => None,
        }
    }
}