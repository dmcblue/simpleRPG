// std
use std::collections::HashMap;

// ext
use serde::{Serialize, Deserialize};

// int
use super::action::ActionType;
use super::data::{Components, Items};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Field {
	LOCATION
}

impl TryFrom<&str> for Field {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"location" => Ok(Field::LOCATION),
			_ => Err("Unknown field")
		}
    }
}

impl TryFrom<Field> for &str {
    type Error = &'static str;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
		match value {
			Field::LOCATION => Ok("location")
		}
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SaveStateChange {
	entity_uuid: usize,
	field: Field,
	value: usize, // uuid
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Save {
	name: String,
	current_location_uuid: usize,
	// item_uuid -> quantity
	inventory: HashMap<usize, usize>,
	state_changes: Vec<SaveStateChange>,
}

#[derive(Debug)]
pub struct ConversationPointer {
	pub conversation_uuid: usize,
	pub path: Vec<usize>,
}

pub struct State {
	pub current_conversation: ConversationPointer,
	pub current_location_uuid: usize,
	pub current_vending_uuid: usize,
	pub last_action_type: ActionType,
	pub state_changes: HashMap<usize, HashMap<Field, usize>>, // what about strings?
}

impl State {
	pub fn state_changes_to_save_state_changes(&self) -> Vec<SaveStateChange> {
		let mut save_state_changes: Vec<SaveStateChange> = Vec::new();

		for (entity_uuid, changes) in self.state_changes.iter() {
			for (field, value) in changes {
				save_state_changes.push(
					SaveStateChange{
						entity_uuid: *entity_uuid,
						field: *field,
						value: *value, // uuid
				
					}
				);
			}
		}

		return save_state_changes;
	}

	pub fn state_changes_to_file_content(&mut self, name: String, components: &mut Components) -> String {
		let save = Save{
			name: name,
			current_location_uuid: self.current_location_uuid,
			inventory: components.get_inventory().to_hash_map(),
			state_changes: self.state_changes_to_save_state_changes(),
		};

		return serde_saphyr::to_string(&save).unwrap();
	}

	pub fn load_from_file(&mut self, contents: String, components: &mut Components) {
		self.state_changes.drain();
		components.get_inventory().drain();

		let save: Save = serde_saphyr::from_str(&contents).unwrap();
		self.current_location_uuid = save.current_location_uuid;
		for (item_uuid, quantity) in save.inventory {
			components.get_inventory().add(
				item_uuid,
				quantity
			);
		}
		for save_state_change in save.state_changes {
			match save_state_change.field {
				Field::LOCATION => {
					// this makes no sense
					self.update_location(save_state_change.entity_uuid, save_state_change.value);
				}
			}
		}
	}

	pub fn update_location(&mut self, entity_uuid: usize, new_value_uuid: usize) {
		if !self.state_changes.contains_key(&entity_uuid) {
			self.state_changes.insert(entity_uuid, HashMap::new());
		}

		self.state_changes.get_mut(&entity_uuid).unwrap().insert(Field::LOCATION, new_value_uuid);
	}
}
