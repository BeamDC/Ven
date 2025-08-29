use crate::bindings;
use crate::constants::DEFAULT_WINDOW_POS;
use crate::core::components::dialogue_tree::DialogueTree;
use crate::core::ui::node_tile::NodeTile;

pub struct NodeTextEdit {
    pub name: String,
    pub title: Option<String>,
    pub content: String,
}

impl NodeTextEdit {
    pub fn new(node: DialogueTree) -> NodeTextEdit {
        let name = match node.clone() {
            DialogueTree::Player { .. } => String::from("Player"),
            DialogueTree::NPC { .. } => String::from("NPC"),
            DialogueTree::Story { .. } => String::from("Story"),
        };

        let title = match node.clone() {
            DialogueTree::Player { .. } => Some(String::from("Player")),
            DialogueTree::NPC { speaker, .. } => Some(speaker.to_string()),
            DialogueTree::Story { .. } => None,
        };

        let content = match node.clone() {
            DialogueTree::Player { text, .. } => text,
            DialogueTree::NPC { text, .. } => text,
            DialogueTree::Story { text, .. } => text,
        };

        NodeTextEdit {
            name,
            title,
            content,
        }
    }

    pub fn display(&mut self) -> (Option<String>, String) {
        bindings::ui(|egui_ctx| {
            egui::Window::new(&self.name)
                .default_pos(DEFAULT_WINDOW_POS)
                .show(egui_ctx, |mut ui| {
                    // title editor
                    // if self.title.is_some() {
                    //     let mut title = self.title.clone().unwrap();
                    //     let response = ui.add(egui::TextEdit::singleline(&mut title));
                    //     if response.changed() {
                    //         self.title = Some(title);
                    //     }
                    // }

                    // content editor
                    // let mut content = self.content.clone();
                    // let response = ui.add(egui::TextEdit::singleline(&mut content));
                    // if response.changed() {
                    //     self.content = content;
                    // }

                    // title editor
                    if let Some(ref mut title) = self.title {
                        ui.add(egui::TextEdit::singleline(title));
                    }

                    // content editor
                    ui.add(egui::TextEdit::singleline(&mut self.content));
                });
        });

        bindings::draw();
        (self.title.clone(), self.content.clone())
    }
}