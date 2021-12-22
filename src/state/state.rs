//! State management abstraction.
//!
//! This can use HTML, JSON, TOML, YAML, etc.

use std::any::Any;
use crate::state::state_enum::StateEnum;

pub trait State: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn to_state_enum(&self) -> StateEnum;
    fn insert(&mut self, key: String, value: String) -> ();
}
