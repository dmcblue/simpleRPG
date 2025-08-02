use super::action::{Action, ActionType};

pub struct Scene {
	pub location_id: usize,
	pub entity_ids: Vec<usize>,
	pub exit_ids: Vec<usize>,
	pub takeable_item_ids: Vec<usize>,
	pub actions: Vec<Action>,
}