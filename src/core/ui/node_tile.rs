use std::fmt::format;
use std::path::PathBuf;
use std::time::Instant;
use macroquad::color::{Color, WHITE};
use macroquad::prelude::{draw_rectangle_ex, measure_text, Font, TextParams, BLACK};
use macroquad::shapes::{draw_rectangle_lines_ex, DrawRectangleParams};
use macroquad::text::draw_text_ex;
use crate::constants::{NODE_MANAGER_LINES, NPC_NODE_OUTLINE, PANEL_BG_FILL, PANEL_OUTLINE_FILL, PLAYER_NODE_OUTLINE, STORY_NODE_OUTLINE};
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::draggable::{Drag, Draggable};
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;

#[derive(Clone)]
pub struct NodeTile<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub last_click: Option<Instant>,

    pub node: DialogueTree,

    pub fill: Color,
    pub outline: Color,
    pub drag: Drag,
    pub text_color: Color,

    pub font: Option<&'a Font>,
    pub font_size: u16,
}

impl Pos for NodeTile<'_> {
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

impl Object for NodeTile<'_> {
    fn get_icon(&self) -> Option<PathBuf> {
        None
    }

    fn get_thickness(&self) -> f32 {
        self.thickness
    }

    fn get_text(&self) -> String {
        match self.node.clone() {
            DialogueTree::Player { text, .. } => text,
            DialogueTree::NPC { text, .. } => text,
            DialogueTree::Story { text, .. } => text
        }
    }

    fn get_font(&self) -> Option<&Font> {
        self.font
    }

    fn get_font_size(&self) -> u16 {
        self.font_size
    }

    fn get_text_colour(&self) -> Color {
        self.text_color
    }

    fn draw_text(&self) {
        // draw detail text
        self.draw_detail();

        // draw main text
        self.draw_content();
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

        // draw text and other details
        draw_rectangle_lines_ex(
            self.x, self.y,
            self.width, self.height / 4.0,
            self.thickness,
            DrawRectangleParams {
                offset: Default::default(),
                rotation: 0.0,
                color: STORY_NODE_OUTLINE,
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

        // draw connection points
        self.draw_connections();

        self.draw_text();
    }
}

impl MouseInteract for NodeTile<'_> {}

impl Draggable for NodeTile<'_> {
    fn get_drag(&self) -> &Drag {
        &self.drag
    }

    fn get_drag_state_mut(&mut self) -> &mut Drag {
        &mut self.drag
    }
}

impl NodeTile<'_> {
    pub fn new(x: f32, y: f32, width: f32, height: f32, node: DialogueTree) -> NodeTile<'static> {
        let outline = match node.clone() {
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
            last_click: None,
            node,
            fill: PANEL_BG_FILL,
            outline,
            drag: Drag::default(),
            text_color: WHITE,
            font: None,
            font_size: 32,
        }
    }

    fn draw_detail(&self) {
        let detail = match self.node.clone() {
            DialogueTree::Player { .. } => "Player",
            DialogueTree::NPC { speaker, ..} => {
                &*format!("Speaker: {}", speaker)
            },
            DialogueTree::Story { .. } => "Story"
        };


        // shrink detail to fit
        let mut dim = measure_text(
            detail,
            self.font,
            self.font_size,
            1.0,
        );

        let mut font_size = self.get_font_size();
        while dim.width >= self.width || dim.height >= self.height / 4.0 {
            font_size -= 1;
            dim = measure_text(
                detail,
                self.font,
                font_size,
                1.0,
            );
        }

        let pad_x = self.x + self.get_thickness();
        let pad_y = self.y + self.height / 4.0 - (font_size / 4) as f32;

        let params = TextParams {
            font: self.font,
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: self.text_color,
        };

        draw_text_ex(
            detail,
            pad_x,
            pad_y,
            params
        );
    }

    fn draw_content(&self) {
        let content = &*match self.node.clone() {
            DialogueTree::Player { text, .. } => text,
            DialogueTree::NPC { text, .. } => text,
            DialogueTree::Story { text, .. } => text
        };

        // shrink detail to fit
        let mut dim = measure_text(
            content,
            self.font,
            self.font_size,
            1.0,
        );

        // todo : insert newlines when text gets too long for the width
        //  after adding newlines, if text still doesnt fit shrink it and reapply newlines.
        //  if text is still to big after a threshold size,
        //  clamp it to that size and instead add "..." at some point to cut it off
        let mut font_size = self.get_font_size();
        while dim.width >= self.width || dim.height >= self.height / 4.0 {
            font_size -= 1;
            dim = measure_text(
                content,
                self.font,
                font_size,
                1.0,
            );
        }

        let pad_x = self.x + self.get_thickness();
        let pad_y = self.y + self.height / 4.0 + dim.height + self.get_thickness();

        let params = TextParams {
            font: self.font,
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: self.text_color,
        };

        draw_text_ex(
            content,
            pad_x,
            pad_y,
            params
        );
    }

    fn draw_connections(&self) {
        todo!("
        draw connection points for nodes at the midpoint of each vertical edge,
        draw lines between the connection points of nodes which are connected");
    }
}