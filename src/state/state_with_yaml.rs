use std::any::Any;
use crate::state::state_trait::StateTrait;
use crate::state::state_enum::StateEnum;

pub type StateWithYAML = ::serde_yaml::Mapping;

impl StateTrait for StateWithYAML {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_state_enum(&self) -> StateEnum {
        StateEnum::StateWithYAML(self.clone())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(&::serde_yaml::Value::String(String::from(key)))
    }
         
    fn insert(&mut self, key: String, value: String) -> () {
        self.insert(::serde_yaml::Value::String(String::from(key)), ::serde_yaml::Value::String(value));
    }

}
