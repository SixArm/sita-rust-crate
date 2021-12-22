use std::any::Any;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithYAML = ::serde_yaml::Value;

impl State for StateWithYAML {

    /// Reflection.
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Convert from this specific state to a state enum.
    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithYAML(self.clone())
    }

    /// Insert.
    fn insert(&mut self, key: String, value: String) -> () {
        match self {
            ::serde_yaml::Value::Mapping(map) => {
                map.insert(::serde_yaml::Value::String(key), ::serde_yaml::Value::String(value));
            },
            _ => {
                panic!("StateWithYAML insert");
            }
        }
    }

}
