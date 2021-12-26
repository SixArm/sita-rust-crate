use std::any::Any;
use crate::types::*;
use crate::state::state_trait::StateTrait;
use crate::state::state_enum::StateEnum;

pub type StateWithMap = Map<String, String>;

impl StateTrait for StateWithMap {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithMap(self.clone())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(key, value);
    }
    
}
