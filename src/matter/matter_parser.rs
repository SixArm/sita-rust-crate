pub trait MatterParser {
    fn parse(text: &str) -> Option<(&str, &str)>;
    fn parse_to_matter_state<S: AsRef<str> + Sized>(text: S) -> crate::matter::state::State;
}
