// int
use super::components::Components;
use super::conversations::ConversationNode;
use super::items::Items;
use super::vending::Vending;

impl Components<'_> {
	pub fn get_uuid(&self, array_index: usize) -> usize {
		return self.uuids[array_index];
	}

	pub fn get_conversation(&self, uuid: usize) -> &ConversationNode {
		return &self.conversations[self.uuid_map.get(&uuid).unwrap() - self.conversations_start];
	}

	pub fn get_conversation_by_speaker(&self, uuid: usize) -> Option<&ConversationNode> {
		match self.owns_conversation[uuid] {
			Some(conversation_uuid) => {
				return Some(self.get_conversation(conversation_uuid));
			},
			None => {
				return None;
			}
		}
	}

	pub fn get_description(&self, uuid: usize) -> String {
		return String::from(self.descriptions[self.get_array_id(uuid)]);
	}

	pub fn get_destination(&self, uuid: usize) -> usize {
		let index = self.get_array_id(uuid);
		return self.destinations[index - self.exits_start];
	}

	pub fn get_name(&self, uuid: usize) -> String {
		return String::from(self.names[self.get_array_id(uuid)]);
	}

	pub fn get_vending(&mut self, uuid: usize) -> &mut Vending {
		return self.vendings.get_mut(&uuid).unwrap();
	}

	pub fn read_vending(&self, uuid: usize) -> &Vending {
		return self.vendings.get(&uuid).unwrap();
	}

	pub fn get_location(&self, uuid: usize) -> &Vec<usize> {
		return &self.locations[self.get_array_id(uuid)];
	}

	pub fn get_inventory(&mut self) -> &mut Items {
		return self.get_location_items(self.inventory_uuid);
	}

	pub fn get_location_items(&mut self, uuid: usize) -> &mut Items {
		return &mut self.location_items[self.get_array_id(uuid)];
	}

	pub fn read_location_items(&self, uuid: usize) -> &Items {
		return &self.location_items[self.get_array_id(uuid)];
	}

	pub fn get_array_id(&self, uuid: usize) -> usize {
		// return *self.uuid_map.get(uuid_ref).unwrap();
		match self.uuid_map.get(&uuid) {
			Some(array_id_ref) => { return *array_id_ref; },
			None => panic!("No array_id for uuid {}", uuid),
		}
	}

	// pub fn has_conversation(&self, uuid: usize) -> bool {
	// 	match self.owns_conversation[self.get_array_id(uuid)] {
	// 		Some(conversation_id) => {
	// 			return true;
	// 		},
	// 		None => {
	// 			return false;
	// 		}
	// 	}
	// }

	pub fn is_enabled(&self, uuid: usize) -> bool {
		match self.enabled.get(&self.get_array_id(uuid)) {
			Some(b) => { return *b; }
			None => { return false; }
		}
	}

	pub fn set_enabled(&mut self, uuid: usize, is_enabled: bool) {
		self.enabled.insert(
			uuid,
			is_enabled
		);
	}

	pub fn is_exit(&self, uuid: usize) -> bool {
		let id = self.get_array_id(uuid);
		if id < self.exits_start {
			return false;
		}

		self.destinations.len() > id - self.exits_start
	}

	pub fn is_vendor(&self, id: usize) -> bool {
		if id < self.people_start || id >= self.exits_start {
			return false;
		}

		match self.owns_vending[id] {
			Some(_) => { true },
			None => { false }
		}
	}

	pub fn is_speaker(&self, id: usize) -> bool {
		if id < self.people_start || id >= self.exits_start {
			return false;
		}

		match self.owns_conversation[id] {
			None => { return false; },
			Some(conversation_id) => {
				return self.conversations[conversation_id].enabled;
			}
		}
	}

	pub fn is_takeable_item(&self, uuid: usize) -> bool {
		let id = self.get_array_id(uuid);
		if id < self.items_start || id >= self.people_start {
			return false;
		}
		self.takeable[id - self.items_start]
	}

	pub fn move_item_to(&mut self, entity_uuid: usize, new_location_uuid: usize) {
		let starting_location_id = self.location_items.iter().position(|items| items.any(entity_uuid) ).unwrap();
		let _ = self.location_items[starting_location_id].remove(entity_uuid, 1);
		let new_location_index = self.get_array_id(new_location_uuid);
		let _ = self.location_items[new_location_index].add(entity_uuid, 1);
	}

	pub fn move_to(&mut self, entity_uuid: usize, new_location_uuid: usize) {
		let entity_index = self.get_array_id(entity_uuid);
		let starting_location_index = self.location_map[entity_index];
		let index = self.locations[starting_location_index].iter().position(|eid| *eid == entity_uuid).unwrap(); // ?
		// remove (index)
		self.locations[starting_location_index].remove(index);
		let new_location_index = self.get_array_id(new_location_uuid);
		self.location_map[entity_index] = new_location_uuid;
		self.locations[new_location_index].push(entity_uuid);
	}
}
