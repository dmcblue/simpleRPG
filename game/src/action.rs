#[derive(PartialEq, Clone)]
pub enum ActionType {
	CHECK_INVENTORY,
	GO,
	LOOK,
	TAKE,
	TALK
}

pub struct Action {
	pub action_type: ActionType,
	pub arg_1: Option<usize>,
	// arg_2: Option<usize>,
	// arg_3: Option<usize>,
}
impl Default for Action {
	fn default() -> Action {
		Action {
			action_type: ActionType::LOOK,
			arg_1: None,
			// arg_2: None,
			// arg_3: None,
		}
	}
}