use std::any::Any;
use crate::types::*;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithHTML = Map<String, String>;

impl State for StateWithHTML {

    /// Reflection.
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Convert from this specific state to a state enum.
    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithHTML(self.clone())
    }

    /// Insert.
    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(key, value);
    }

}
