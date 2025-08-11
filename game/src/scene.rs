use super::action::{Action};

pub struct Scene {
	pub location_id: usize,
	pub entity_ids: Vec<usize>,
	pub actions: Vec<Action>,
}
