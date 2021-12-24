use std::any::Any;
use crate::types::*;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithHTML = Map<String, String>;

impl State for StateWithHTML {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithHTML(self.clone())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(key, value);
    }
    
}
