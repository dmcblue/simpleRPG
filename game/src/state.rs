use std::collections::HashMap;
use super::action::ActionType;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Field {
	LOCATION
}

impl TryFrom<&str> for Field {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"location" => Ok(Field::LOCATION),
			_ => Err("Unknown fields")
		}
    }
}

pub struct State {
	pub current_location: usize,
	pub last_action_type: ActionType,
	pub state_changes: HashMap<usize, HashMap<Field, usize>>, // what about strings?
}

impl State {
	pub fn update_location(&mut self, entity_uuid: usize, new_value: usize) {
		// let key = format!("{}:{}", entity_uuid, Field::LOCATION.try_into(&str))
		if !self.state_changes.contains_key(&entity_uuid) {
			self.state_changes.insert(entity_uuid, HashMap::new());
		}

		self.state_changes.get_mut(&entity_uuid).unwrap().insert(Field::LOCATION, new_value);
	}
}