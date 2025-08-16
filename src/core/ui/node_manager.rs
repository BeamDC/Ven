use crate::constants::{NODE_HIGHLIGHT, NODE_SELECT_OUTLINE, NODE_MANAGER_FILL, NODE_MANAGER_LINES, NPC_NODE_OUTLINE, PANEL_BG_FILL, PANEL_OUTLINE_FILL, PLAYER_NODE_OUTLINE, STORY_NODE_OUTLINE, DOUBLE_CLICK_DELAY_MS};
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::interaction::{MouseInteract, Pos};
use crate::core::traits::object::Object;
use crate::core::ui::node_tile::NodeTile;
use macroquad::color::{Color, BLACK};
use macroquad::prelude::{draw_rectangle_ex, DrawRectangleParams, Font};
use std::path::PathBuf;
use std::time::Instant;
use macroquad::input::MouseButton;
use crate::core::traits::draggable::Draggable;
use crate::core::ui::node_manager::NodeAction::{RemoveIndex, SelectIndex};

pub enum NodeAction {
    RemoveIndex(usize),
    SelectIndex(usize),
}

pub struct NodeManager<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub zoom: f32,

    pub fill: Color,
    pub outline: Color,
    pub lines: Color,

    pub nodes: Vec<NodeTile<'a>>,
    pub selected: Option<usize>,
}

impl Pos for NodeManager<'_> {
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

impl Object for NodeManager<'_> {
    fn get_icon(&self) -> Option<PathBuf> {
        None
    }

    fn get_thickness(&self) -> f32 {
        todo!()
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
        for  (i, mut node) in self.nodes.clone().iter_mut().enumerate() {
            node.x = node.x.clamp(self.x, self.x + self.width - node.width);
            node.y = node.y.clamp(self.y, self.y + self.height - node.height);

            node.outline = match node.node {
                DialogueTree::Player { .. } => PLAYER_NODE_OUTLINE,
                DialogueTree::NPC { .. } => NPC_NODE_OUTLINE,
                DialogueTree::Story { .. } => STORY_NODE_OUTLINE,
            };
            node.fill = PANEL_BG_FILL;
            // todo : only highlight the top one
            node.on_hover_mut(|nt| {
                nt.fill = NODE_HIGHLIGHT;
            });
            if self.selected.is_some() && i == self.selected.unwrap() {
                node.outline = NODE_SELECT_OUTLINE;
            }

            node.connections.0.x = node.x;
            node.connections.0.y = node.y + node.height / 2.0;

            node.connections.1.x = node.x + node.width;
            node.connections.1.y = node.y + node.height / 2.0;

            node.draw();
        }
    }
}

impl MouseInteract for NodeManager<'_> {}

impl NodeManager<'_> {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> NodeManager<'static> {
        NodeManager {
            x,
            y,
            width,
            height,
            thickness: 1.0,
            zoom: 1.0,
            fill: NODE_MANAGER_FILL,
            outline: PANEL_OUTLINE_FILL,
            lines: NODE_MANAGER_LINES,
            nodes: vec![],
            selected: None
        }
    }

    pub fn handle_inputs(&mut self) -> Option<NodeAction> {
        for node in &mut self.nodes.iter_mut().rev() {
            if node.update_drag() {break}
        }

        // node selection
        for (i, mut node) in self.nodes.iter_mut().enumerate().rev(){
            if node.is_pressed(MouseButton::Left) {
                let time = Instant::now();
                if let Some(last) = node.last_click {
                    if last.elapsed().as_millis() <= DOUBLE_CLICK_DELAY_MS {
                         return Some(SelectIndex(i))
                    }
                }
                node.last_click = Some(time);
            }
        }

        // node deletion
        for (i, node) in self.nodes.iter().enumerate().rev() {
            if node.is_pressed(MouseButton::Right) {
                return Some(RemoveIndex(i))
            }
        }

        None
    }
}