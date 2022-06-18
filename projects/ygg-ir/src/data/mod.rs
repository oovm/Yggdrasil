use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    ops::{Range, RangeInclusive},
};

use num::BigInt;
use serde::{Deserialize, Serialize};

use character_set::CharacterSet;
use regex_automata::dfa::regex::Regex;

use crate::{
    data::charset::{char_range_display, char_set_display, string_display},
    *,
};

pub mod builtin;
pub mod charset;
pub mod rule_ref;
mod serder;
pub mod symbol;

//
#[derive(Debug, Clone)]
pub enum DataKind {
    Ignored,
    Boolean(bool),
    Integer(BigInt),
    String(String),
    StringFused(Regex),
    CharacterAny,
    Character(char),
    CharacterRange(RangeInclusive<char>),
    CharacterBuiltin(String),
    CharacterFused(CharacterSet),
}

impl Display for DataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Ignored => write!(f, "IGNORED"),
            DataKind::Boolean(v) => write!(f, "{}", v),
            DataKind::Integer(v) => write!(f, "{}", v),
            DataKind::String(v) => string_display(v, f),
            DataKind::StringFused(_) => write!(f, "FUSED_STRING"),
            DataKind::CharacterAny => write!(f, "ANY"),
            DataKind::Character(c) => write!(f, "{:?}", c),
            DataKind::CharacterRange(range) => char_range_display(range, f),
            DataKind::CharacterBuiltin(set) => write!(f, "{}", set),
            DataKind::CharacterFused(set) => char_set_display(set, f),
        }
    }
}

impl From<DataKind> for ExpressionNode {
    fn from(value: DataKind) -> Self {
        Self { tag: String::new(), kind: ExpressionKind::Data(Box::new(value)) }
    }
}

impl DataKind {}
