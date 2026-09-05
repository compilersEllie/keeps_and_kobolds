use crokey::KeyCombination;
use crossterm::event::{KeyModifiers, MouseEventKind};
use derive_more::with_trait::{AsMut, AsRef, Deref, From};
use serde::{Deserialize, Serialize};

use crate::effects::{Condition, Effect};
use crate::item::Item;
use crate::typed_id::Id;

type LineCode = (String, usize); // Language file prefix, line

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UseAs {
    PlayerNickname,
    CharacterNickname,
    CodeWord {
        phrase_code: String,
    },
    CheckCodeWord {
        phrase_code: String,
        on_success: Label,
        on_failure: Label,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionKind {
    If(Condition, Label),
    Effect(Effect), // Steal, Give, Buff, Debuff, Give goals etc

    // NPC only:
    Choice(Vec<Label>),
    Question(LineCode, UseAs),
    Trade {
        request: Item,
        reward: Item,
        on_success: Label,
        on_failure: Label,
    },
    Shop,
    Label(Label), // To jump to.
    Speak(LineCode),
    // Sub discussion
    Discussion(Id<Discussion>),
}

#[derive(
    From,
    AsMut,
    Deref,
    AsRef,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MouseCombination {
    #[deref]
    mouse: MouseEventKind,
    mods: KeyModifiers,
}

impl PartialOrd for MouseCombination {
    fn partial_cmp(&self, other: &MouseCombination) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MouseCombination {
    fn cmp(&self, other: &MouseCombination) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let mouse = match (self.mouse, other.mouse) {
            (MouseEventKind::Down(s), MouseEventKind::Down(o)) => s.partial_cmp(&o).unwrap(),
            (MouseEventKind::Down(_), _) => Less,
            (_, MouseEventKind::Down(_)) => Greater,
            (MouseEventKind::Up(s), MouseEventKind::Up(o)) => s.partial_cmp(&o).unwrap(),
            (MouseEventKind::Up(_), _) => Less,
            (_, MouseEventKind::Up(_)) => Greater,
            (MouseEventKind::Drag(s), MouseEventKind::Drag(o)) => s.partial_cmp(&o).unwrap(),
            (MouseEventKind::Drag(_), _) => Less,
            (_, MouseEventKind::Drag(_)) => Greater,
            (MouseEventKind::Moved, MouseEventKind::Moved) => Equal,
            (MouseEventKind::Moved, _) => Less,
            (_, MouseEventKind::Moved) => Greater,
            (MouseEventKind::ScrollDown, MouseEventKind::ScrollDown) => Equal,
            (MouseEventKind::ScrollDown, _) => Less,
            (_, MouseEventKind::ScrollDown) => Greater,
            (MouseEventKind::ScrollUp, MouseEventKind::ScrollUp) => Equal,
            (MouseEventKind::ScrollUp, _) => Less,
            (_, MouseEventKind::ScrollUp) => Greater,
            (MouseEventKind::ScrollLeft, MouseEventKind::ScrollLeft) => Equal,
            (MouseEventKind::ScrollLeft, _) => Less,
            (_, MouseEventKind::ScrollLeft) => Greater,
            (MouseEventKind::ScrollRight, MouseEventKind::ScrollRight) => Equal,
        };
        mouse.then(self.mods.partial_cmp(&other.mods).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Action {
    pub name: String,
    pub kind: ActionKind,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_bind: Vec<KeyCombination>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_mouse: Vec<MouseCombination>,
}

type Label = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Discussion {
    // TODO(feat): Implement discussion #3
    // TODO(feat): Implement internationalisation #5
    pub starts: Vec<Label>,
    pub current: Option<usize>,
    pub lines: Vec<Action>,
}
