#[derive(PartialEq, Clone, Copy)]
pub enum ActionType {
	CHALLENGE,
	CHECK_INVENTORY,
	GO,
	LOOK,
	TAKE,
	TALK,
	VEND,
}

#[derive(Clone, Copy)]
pub struct Action {
	pub action_type: ActionType,
	pub arg_1: Option<usize>,
	pub arg_2: Option<usize>,
	// arg_3: Option<usize>,
}

impl Default for Action {
	fn default() -> Action {
		Action {
			action_type: ActionType::LOOK,
			arg_1: None,
			arg_2: None,
			// arg_3: None,
		}
	}
}
