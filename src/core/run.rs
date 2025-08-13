use crate::core::traits::object::Object;
use crate::constants::{NODE_TILE_HEIGHT, NODE_TILE_WIDTH, SCENE_VIEWER_WIDTH, TOOLBAR_HEIGHT};
use crate::core::ui::scene_viewer::SceneViewer;
use crate::core::ui::toolbar::{ToolbarAction, Toolbar};
use macroquad::prelude::*;
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::draggable::Draggable;
use crate::core::ui::node_manager::{NodeAction, NodeManager};
use crate::core::ui::node_tile::NodeTile;

pub async fn run() {
    let mut toolbar = Toolbar::new(screen_width(), TOOLBAR_HEIGHT);
    let mut scene_viewer = SceneViewer::new(
        0.0, toolbar.height,
        SCENE_VIEWER_WIDTH, screen_height() - toolbar.height
    );
    let mut node_manager = NodeManager::new(
        scene_viewer.width, toolbar.height,
        screen_width() - scene_viewer.width, screen_height() - toolbar.height
    );

    loop {
        // scale with screen
        toolbar.width = screen_width();
        scene_viewer.height = screen_height() - toolbar.height;
        node_manager.width = screen_width() - scene_viewer.width;
        node_manager.height = screen_height() - toolbar.height;
        clear_background(BLACK);

        // toolbar action handling
        let toolbar_action = toolbar.handle_input();
        let text = String::from("Enter text here...");
        match toolbar_action {
            Some(ToolbarAction::AddPlayer) => {
                let node = DialogueTree::Player {
                    from: vec![],
                    text,
                    next: vec![],
                };
                let tile = NodeTile::new(
                    node_manager.x, node_manager.y,
                    NODE_TILE_WIDTH, NODE_TILE_HEIGHT,
                    node
                );
                node_manager.nodes.push(tile);
            }
            Some(ToolbarAction::AddNpc) => {
                let node = DialogueTree::NPC {
                    from: vec![],
                    speaker: "".to_string(),
                    text,
                    next: vec![],
                };
                let tile = NodeTile::new(
                    node_manager.x, node_manager.y,
                    NODE_TILE_WIDTH, NODE_TILE_HEIGHT,
                    node
                );
                node_manager.nodes.push(tile);
            }
            Some(ToolbarAction::AddStory) => {
                let node = DialogueTree::Story {
                    from: vec![],
                    text,
                };
                let tile = NodeTile::new(
                    node_manager.x, node_manager.y,
                    NODE_TILE_WIDTH, NODE_TILE_HEIGHT,
                    node
                );
                node_manager.nodes.push(tile);
            }
            None => {}
        }

        // node manager action handling
        let node_action = node_manager.handle_inputs();
        match node_action {
            Some(NodeAction::RemoveIndex(i)) => {
                node_manager.nodes.remove(i);
            }
            Some(NodeAction::SelectIndex(i)) => {
                if node_manager.selected.is_some() &&
                    i == node_manager.selected.unwrap() {
                    node_manager.selected = None;
                } else {
                    node_manager.selected = Some(i);
                }
            }
            None => {}
        }

        // rendering
        scene_viewer.draw();
        toolbar.draw();
        node_manager.draw();

        next_frame().await
    }
}