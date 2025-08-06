use crate::core::traits::object::Object;
use crate::constants::{NODE_TILE_HEIGHT, NODE_TILE_WIDTH, SCENE_VIEWER_WIDTH, TOOLBAR_HEIGHT};
use crate::core::ui::scene_viewer::SceneViewer;
use crate::core::ui::toolbar::{Action, Toolbar};
use macroquad::prelude::*;
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::traits::draggable::Draggable;
use crate::core::ui::node_manager::NodeManager;
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

        // action handling
        let action = toolbar.handle_input();
        match action {
            Some(Action::AddPlayer) => {
                let node = DialogueTree::Player {
                    text: "".to_string(),
                    next: vec![],
                };
                let tile = NodeTile::new(
                    node_manager.x, node_manager.y,
                    NODE_TILE_WIDTH, NODE_TILE_HEIGHT,
                    node
                );
                node_manager.nodes.push(tile);
            }
            Some(Action::AddNpc) => {
                let node = DialogueTree::NPC {
                    speaker: "".to_string(),
                    text: "".to_string(),
                    next: vec![],
                };
                let tile = NodeTile::new(
                    node_manager.x, node_manager.y,
                    NODE_TILE_WIDTH, NODE_TILE_HEIGHT,
                    node
                );
                node_manager.nodes.push(tile);
            }
            Some(Action::AddStory) => {
                let node = DialogueTree::Story {
                    text: "".to_string(),
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

        // rendering
        scene_viewer.draw();
        toolbar.draw();

        for node in &mut node_manager.nodes {
            node.update_drag();
        }
        node_manager.draw();

        next_frame().await
    }
}