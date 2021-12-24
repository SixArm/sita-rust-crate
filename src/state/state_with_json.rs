use std::any::Any;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithJSON = ::serde_json::Map<String, ::serde_json::Value>;

impl State for StateWithJSON {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithJSON(self.clone())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }
            
    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(key, ::serde_json::Value::String(value));
    }
    
}