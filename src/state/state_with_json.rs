use std::any::Any;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithJSON = ::serde_json::Value;

impl State for StateWithJSON {

    /// Reflection.
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Convert from this specific state to a state enum.
    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithJSON(self.clone())
    }

    /// Insert    
    fn insert(&mut self, key: String, value: String) -> () {
        match self {
            ::serde_json::Value::Object(map) => {
                map.insert(key, ::serde_json::Value::String(value));
            },
            _ => {
                panic!("StateWithJSON insert");
            }
        }
    }

}