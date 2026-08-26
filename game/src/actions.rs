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
pub enum Action {
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

type Label = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Discussion {
    // TODO: Implement discussion
    // TODO: Implement internationalisation
    pub starts: Vec<Label>,
    pub current: Option<usize>,
    lines: Vec<Action>,
}
