use std::any::Any;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithTOML = ::toml::value::Table;

impl State for StateWithTOML {
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithTOML(self.clone())
    }

    fn has_key(&self, key: &str) -> bool {
        self.has_key(key)
    }
 
    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(key, ::toml::Value::String(value));
    }

}
