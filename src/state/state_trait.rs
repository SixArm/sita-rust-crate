//! State management abstraction.
//!
//! This can use HTML, JSON, TOML, YAML, etc.
//! 
//! This can be expanded for potential future formats.

use std::any::Any;
use crate::state::state_enum::StateEnum;

pub trait StateTrait: std::fmt::Debug {

    // Convert from this specific state to any type.
    fn as_any(&self) -> &dyn Any;

    // Convert from this specific state to a state enum.
    fn to_state_enum(&self) -> StateEnum;

    // Does the state contain the key?
    fn contains_key(&self, key: &str) -> bool;

    // Insert the key and value.
    fn insert(&mut self, key: String, value: String) -> ();

    /// If the key doesn't exist then insert the key and value.
    fn contains_key_or_insert(&mut self, key: String, value: String) -> () {
        if !self.contains_key(&key) { 
            self.insert(key, value); 
        }
    }

}
