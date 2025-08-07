use crate::core::components::ptr::P;

/// the dialogue nodes for the tree
#[derive(Clone)]
pub enum DialogueTree {
    /// for the dialogue of the player character
    Player {
        from: Vec<P<DialogueTree>>,
        text: String,
        next: Vec<P<DialogueTree>>,
    },
    /// for all other (non-player) characters
    NPC {
        from: Vec<P<DialogueTree>>,
        speaker: String,
        text: String,
        next: Vec<P<DialogueTree>>,
    },
    /// for describing the actions during dialogue
    Story {
        from: Vec<P<DialogueTree>>,
        text: String,
    }
}