use super::action::{Action};

pub struct Scene {
	pub location_uuid: usize,
	pub entity_uuids: Vec<usize>,
	pub actions: Vec<Action>,
}
