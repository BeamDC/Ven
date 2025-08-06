use std::path::PathBuf;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle_ex, Font, BLACK};
use macroquad::shapes::{draw_rectangle_lines_ex, DrawRectangleParams};
use crate::constants::{NPC_NODE_OUTLINE, PANEL_BG_FILL, PLAYER_NODE_OUTLINE, STORY_NODE_OUTLINE};
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::draggable::{Drag, Draggable};
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;

#[derive(Clone)]
pub struct NodeTile {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,

    pub node: DialogueTree,

    pub fill: Color,
    pub outline: Color,
    pub drag: Drag,
}

impl Pos for NodeTile {
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
        self.x = new;
    }

    fn set_y(&mut self, new: f32) {
        self.y = new;
    }
}

impl Object for NodeTile {
    fn get_border_thickness(&self) -> f32 {
        self.thickness
    }

    fn get_icon(&self) -> Option<PathBuf> {
        None
    }

    fn get_text(&self) -> String {
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
        // draw base rect
        draw_rectangle_ex(
            self.x, self.y,
            self.width, self.height,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: self.fill,
            }
        );

        // draw lines
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

impl MouseInteract for NodeTile {}

impl Draggable for NodeTile {
    fn get_drag_state(&self) -> &Drag {
        &self.drag
    }

    fn get_drag_state_mut(&mut self) -> &mut Drag {
        &mut self.drag
    }
}

impl NodeTile {
    pub fn new(x: f32, y: f32, width: f32, height: f32, node: DialogueTree) -> NodeTile {
        let outline = match node {
            DialogueTree::Player { .. } => PLAYER_NODE_OUTLINE,
            DialogueTree::NPC { .. } => NPC_NODE_OUTLINE,
            DialogueTree::Story { .. } => STORY_NODE_OUTLINE,
        };

        NodeTile {
            x,
            y,
            width,
            height,
            thickness: 2.0,
            node,
            fill: PANEL_BG_FILL,
            outline,
            drag: Drag::default()
        }
    }
}