use std::collections::HashMap;
use super::action::ActionType;
use super::data::Components;
use super::data::ConversationNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

pub struct ConversationPointer {
	pub conversation_id: usize,
	pub path: Vec<usize>,
}

pub struct State {
	pub current_conversation: ConversationPointer,
	pub current_location_id: usize,
	pub last_action_type: ActionType,
	pub state_changes: HashMap<usize, HashMap<Field, usize>>, // what about strings?
}

impl State {
	pub fn state_changes_to_file_content(&self, name: String, components: &Components) -> String {
		let mut contents = String::new();
		contents.push_str(format!("{}\n", name).as_str());
		contents.push_str(format!("{}\n", components.uuids[self.current_location_id]).as_str());
		contents.push_str(
			format!(
				"{}\n",
				components.location_items[components.inventory_id].iter().
							map(|id| format!("{}", components.uuids[*id])).
							collect::<Vec<_>>().
							join(":")
			).as_str()
		);

		for (entity_uuid, changes) in self.state_changes.iter() {
			contents.push_str(format!("{}", entity_uuid).as_str());
			for (field, value) in changes {
				contents.push_str(
					format!(
						";{}:{}",
						<Field as TryInto<&str>>::try_into(*field).unwrap(),
						value
					).as_str()
				);
			}
			contents.push_str("\n");
		}

		return contents;
	}

	pub fn load_from_file(&mut self, contents: String, components: &mut Components) {
		self.state_changes.drain();
		let mut i: usize = 0;
		for line in contents.split("\n") {
			if i == 0 {
				// name
			} else if i == 1 {
				let location_uuid = line.parse::<usize>().unwrap();
				self.current_location_id = components.get_array_id(&location_uuid);
			} else if i == 2 {
				for part in line.split(":") {
					let item_uuid = line.parse::<usize>().unwrap();
					components.location_items[components.inventory_id].push(
						components.get_array_id(&item_uuid)
					);
				}
			} else {
				let mut j: usize = 0;
				let mut entity_id: usize = 0;
				for part in line.split(";") {
					if j == 0 {
						match part.parse::<usize>() {
							Ok(id) => {
								entity_id = components.get_array_id(&id);
							},
							Err(_) => {},
						}
					} else {
						let subparts: Vec<&str> = part.split(":").collect();
						let field = <&str as TryInto<Field>>::try_into(subparts.get(0).unwrap()).unwrap();
						let new_value = subparts.get(1).unwrap().parse::<usize>().unwrap();
						match field {
							Field::LOCATION => {
								// this makes no sense
								self.update_location(entity_id, new_value);
							}
						}
					}

					j = j + 1;
				}
			}
			i = i + 1;
		}
	}

	pub fn update_location(&mut self, entity_uuid: usize, new_value_uuid: usize) {
		if !self.state_changes.contains_key(&entity_uuid) {
			self.state_changes.insert(entity_uuid, HashMap::new());
		}

		self.state_changes.get_mut(&entity_uuid).unwrap().insert(Field::LOCATION, new_value_uuid);
	}
}
