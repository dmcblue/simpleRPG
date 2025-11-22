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
	pub conversation_id: usize,
	pub path: Vec<usize>,
}

pub struct State {
	pub current_conversation: ConversationPointer,
	pub current_location_id: usize,
	pub current_vending_index: usize,
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

	pub fn state_changes_to_file_content(&self, name: String, components: &Components) -> String {
		let save = Save{
			name: name,
			current_location_uuid: components.uuids[self.current_location_id],
			inventory: components.location_items[components.inventory_id].to_hash_map(),
			state_changes: self.state_changes_to_save_state_changes(),
		};

		return serde_saphyr::to_string(&save).unwrap();
	}

	pub fn load_from_file(&mut self, contents: String, components: &mut Components) {
		self.state_changes.drain();
		components.location_items[components.inventory_id].drain();

		let save: Save = serde_saphyr::from_str(&contents).unwrap();
		self.current_location_id = components.get_array_id(&save.current_location_uuid);
		for (item_uuid, quantity) in save.inventory {
			components.location_items[components.inventory_id].add(
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

		// pub struct Save {
		// 	name: String,
		// 	current_location_uuid: usize,
		// 	// item_uuid -> quantity
		// 	inventory: HashMap<usize, usize>,
		// 	state_changes: Vec<SaveStateChange>,
		// }
		// pub struct SaveStateChange {
		// 	entity_uuid: usize,
		// 	field: Field,
		// 	value: usize, // uuid
		// }

		// let mut i: usize = 0;
		// for line in contents.split("\n") {
		// 	if i == 0 {
		// 		// name
		// 	} else if i == 1 {
		// 		let location_uuid = line.parse::<usize>().unwrap();
		// 		self.current_location_id = components.get_array_id(&location_uuid);
		// 	} else if i == 2 {
		// 		for part in line.split(":") {
		// 			let item_uuid = part.parse::<usize>().unwrap();
		// 			components.location_items[components.inventory_id].add(
		// 				// components.get_array_id(&item_uuid),
		// 				item_uuid,
		// 				1
		// 			);
		// 		}
		// 	} else {
		// 		let mut j: usize = 0;
		// 		let mut entity_id: usize = 0;
		// 		for part in line.split(";") {
		// 			if j == 0 {
		// 				match part.parse::<usize>() {
		// 					Ok(id) => {
		// 						entity_id = components.get_array_id(&id);
		// 					},
		// 					Err(_) => {},
		// 				}
		// 			} else {
		// 				let subparts: Vec<&str> = part.split(":").collect();
		// 				let field = <&str as TryInto<Field>>::try_into(subparts.get(0).unwrap()).unwrap();
		// 				let new_value = subparts.get(1).unwrap().parse::<usize>().unwrap();
		// 				match field {
		// 					Field::LOCATION => {
		// 						// this makes no sense
		// 						self.update_location(entity_id, new_value);
		// 					}
		// 				}
		// 			}

		// 			j = j + 1;
		// 		}
		// 	}
		// 	i = i + 1;
		// }
	}

	pub fn update_location(&mut self, entity_uuid: usize, new_value_uuid: usize) {
		if !self.state_changes.contains_key(&entity_uuid) {
			self.state_changes.insert(entity_uuid, HashMap::new());
		}

		self.state_changes.get_mut(&entity_uuid).unwrap().insert(Field::LOCATION, new_value_uuid);
	}
}
