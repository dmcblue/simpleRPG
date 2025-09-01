use super::components::Components;

impl Components<'_> {
	pub fn get_array_id(&self, uuid_ref: &usize) -> usize {
		return *self.uuid_map.get(uuid_ref).unwrap();
	}

	pub fn is_exit(&self, id: usize) -> bool {
		if id < self.exits_start {
			return false;
		}
		self.destinations.contains(&(id - self.exits_start))
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

	pub fn is_takeable_item(&self, id: usize) -> bool {
		if id < self.items_start || id >= self.people_start {
			return false;
		}
		self.takeable[id - self.items_start]
	}

	pub fn move_item_to(&mut self, entity_uuid: usize, new_location_id: usize) {
		let id = self.get_array_id(&entity_uuid);
		let starting_location_id = self.location_items.iter().position(|item_ids| item_ids.contains(&id)).unwrap();
		let index = self.location_items[starting_location_id].iter().position(|item_id| *item_id == id).unwrap();
		self.location_items[starting_location_id].remove(index);
		self.location_items[new_location_id].push(id);
	}

	pub fn move_to(&mut self, entity_uuid: usize, new_location_id: usize) {
		let starting_location_id = self.location_map[entity_uuid];
		let index = self.locations[starting_location_id].iter().position(|eid| *eid == entity_uuid).unwrap();
		self.locations[starting_location_id].remove(index);
		self.location_map[entity_uuid] = new_location_id;
		self.locations[new_location_id].push(entity_uuid);
	}
}
