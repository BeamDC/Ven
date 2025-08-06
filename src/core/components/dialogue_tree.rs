use crate::core::components::ptr::P;

/// the dialogue nodes for the tree
#[derive(Clone)]
pub enum DialogueTree {
    /// for the dialogue of the player character
    Player {
        text: String,
        next: Vec<P<DialogueTree>>,
    },
    /// for all other (non-player) characters
    NPC {
        speaker: String,
        text: String,
        next: Vec<P<DialogueTree>>,
    },
    /// for describing the actions during dialogue
    Story {
        text: String,
    }
}