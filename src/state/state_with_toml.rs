use std::any::Any;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;

pub type StateWithTOML = ::toml::Value;

impl State for StateWithTOML {
    
    /// Reflection.
    fn as_any(&self) -> &dyn Any {
        self
    }

    // Convert from this specific state to a state enum.
    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithTOML(self.clone())
    }

    /// Insert
    fn insert(&mut self, key: String, value: String) -> () {
        match self {
            ::toml::Value::Table(table) => {
                table.insert(key, ::toml::Value::String(value));
            },
            _ => {
                panic!("StateWithTOML insert");
            }
        }
    }

}
