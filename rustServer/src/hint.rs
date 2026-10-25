use crate::types::*;
use crate::mancala_codec::MancalaCodec;
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug, ToSchema)]
pub enum HintType {
    Normal = 0,
    Unknown = 1,
    Completed = 2,
    PrunedCompleted = 3,
    Invalid = 4
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Hint {
    value: Pebbles,
    hint_type: HintType,
    min: Pebbles,
    max: Pebbles
}

pub type HintData = [Hint; 6];

impl Hint {
    pub fn new_empty(pebbles_left: Pebbles) -> Self {
        Self {
            value: 0,
            hint_type: HintType::Unknown,
            min: 0,
            max: pebbles_left
        }
    }

    pub fn new(value: Pebbles, hint_type: HintType, min: Pebbles, max: Pebbles) -> Self {
        Self {
            value,
            hint_type,
            min,
            max
        }
    }

    pub fn new_from_encoded(encoded: u8, min: Pebbles, max: Pebbles) -> Self {
        let (value, hint_type) = MancalaCodec::decode_hint(encoded);
        Self {
            value,
            hint_type,
            min,
            max
        }
    }

    pub fn new_completed(value: Pebbles) -> Self {
        Self {
            value,
            hint_type: HintType::Completed,
            min: value,
            max: value
        }
    }

    pub fn new_invalid() -> Self {
        Self {
            value: 0,
            hint_type: HintType::Invalid,
            min: 0,
            max: 0
        }
    }

    pub fn equals(&self, other: &Hint) -> bool {
        self.value == other.value &&
        self.hint_type == other.hint_type &&
        self.min == other.min &&
        self.max == other.max
    }

    pub fn get_encoded(&self) -> u8 {
        MancalaCodec::encode_hint(self)
    }

    pub fn get_value(&self) -> Pebbles {
        self.value
    }

    pub fn get_type(&self) -> HintType {
        self.hint_type
    }

    pub fn get_min(&self) -> Pebbles {
        self.min
    }

    pub fn get_max(&self) -> Pebbles {
        self.max
    }
}