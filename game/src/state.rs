use super::action::ActionType;

pub struct State {
	pub current_location: usize,
	pub last_action_type: ActionType,
}