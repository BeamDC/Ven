use std::path::PathBuf;
use macroquad::color::{Color, BLACK};
use macroquad::input::MouseButton;
use macroquad::prelude::{draw_rectangle_ex, DrawRectangleParams, Font};
use crate::constants::{NODE_HIGHLIGHT, NODE_HOVER_OUTLINE, NODE_MANAGER_FILL, NPC_NODE_OUTLINE, PANEL_BG_FILL, PANEL_OUTLINE_FILL, PLAYER_NODE_OUTLINE, STORY_NODE_OUTLINE, TOOLBAR_BUTTON_HIGHLIGHT};
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;
use crate::core::ui::node_tile::NodeTile;

pub struct NodeManager {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,

    pub fill: Color,
    pub outline: Color,

    pub nodes: Vec<NodeTile>
}

impl Pos for NodeManager {
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

impl Object for NodeManager {
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
        // draw base rectangle
        draw_rectangle_ex(
            self.x, self.y,
            self.width, self.height,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: self.fill,
            }
        );

        // draw gridlines
        // draw node tiles
        for mut node in self.nodes.clone() {
            node.x += self.x;
            node.y += self.y;

            node.outline = match node.node {
                DialogueTree::Player { .. } => PLAYER_NODE_OUTLINE,
                DialogueTree::NPC { .. } => NPC_NODE_OUTLINE,
                DialogueTree::Story { .. } => STORY_NODE_OUTLINE,
            };
            node.fill = PANEL_BG_FILL;
            node.on_hover_mut(|nt| {
                nt.outline = NODE_HOVER_OUTLINE;
                nt.fill = NODE_HIGHLIGHT;
            });
            node.draw();
        }
    }
}

impl MouseInteract for NodeManager {}

impl NodeManager {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> NodeManager {
        NodeManager {
            x,
            y,
            width,
            height,
            thickness: 1.0,
            fill: NODE_MANAGER_FILL,
            outline: PANEL_OUTLINE_FILL,
            nodes: vec![],
        }
    }
}