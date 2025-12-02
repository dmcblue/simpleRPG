mod components;
mod conversations;
mod counts;
mod entities;
mod main_file;
mod vending;

// std
use std::fs::{
	File,
	metadata, 
	read_dir,
	read_to_string
};
use std::io::Write;
use std::collections::HashMap;
use std::clone::Clone;

// ext

// int
use components::{write_components_file};
use conversations::{
	// ConversationNode, 
	ConversationsFile
};
use counts::Counts;
use entities::{
	Entity,
	ENTITY_TYPE_CONVERSATION,
	ENTITY_TYPE_EXIT,
	ENTITY_TYPE_ITEM,
	ENTITY_TYPE_LOCATION,
	ENTITY_TYPE_PERSON,
	ENTITY_TYPE_VENDING,
};
use main_file::MainFile;
use vending::{
	Vending, 
	VendingsFile, 
	// VendItem
};

fn load_entities_from_dir(builder: &mut Builder, dir_name: &str) {
	let paths = read_dir(dir_name).unwrap();
	for path in paths {
		let file_path = path.unwrap().path();
        if !metadata(&file_path).unwrap().is_dir() {
			let contents = read_to_string(file_path).unwrap();
			let entity: Entity = serde_saphyr::from_str(&contents).unwrap();

			if entity.entity_type == "Game" {
				builder.counts.starting_location_uuid = entity.location.unwrap();
			} else {
				let uuid: usize = entity.id.unwrap();
				match entity.entity_type.as_str() {
					ENTITY_TYPE_CONVERSATION => { builder.counts.conversations.push(uuid); },
					ENTITY_TYPE_EXIT => { builder.counts.exits.push(uuid); },
					ENTITY_TYPE_ITEM => { builder.counts.items.push(uuid); },
					ENTITY_TYPE_LOCATION => { builder.counts.locations.push(uuid); },
					ENTITY_TYPE_PERSON => { builder.counts.people.push(uuid); },
					ENTITY_TYPE_VENDING => { builder.counts.vending.push(uuid); },
					_ => ()
				}

				if entity.metaname == "Inventory" {
					builder.counts.inventory_uuid = uuid;
				} else if entity.metaname == "Vending Ether" {
					builder.counts.vending_ether_uuid = uuid;
				}
				builder.entities.insert(uuid, entity);
			}
		}
	}
}

struct Builder {
	counts: Counts,
	entities: HashMap<usize, Entity>,
	main_file: MainFile,
	uuid_to_index: HashMap<usize, usize>,
	index_to_uuid: HashMap<usize, usize>,
}

impl Builder {
	pub fn add_cache_item(&mut self, entity_uuid: usize) {
		self.uuid_to_index.insert(entity_uuid, self.counts.total);
		self.index_to_uuid.insert(self.counts.total, entity_uuid);
		self.counts.total = self.counts.total + 1;
	}

	pub fn build_cache(&mut self) {
		self.counts.locations_start = self.counts.total;
		for entity_uuid in self.counts.locations.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.locations_end = self.counts.total;
		self.counts.items_start = self.counts.total;
		for entity_uuid in self.counts.items.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.items_end = self.counts.total;
		self.counts.people_start = self.counts.total;
		for entity_uuid in self.counts.people.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.people_end = self.counts.total;
		self.counts.exits_start = self.counts.total;
		for entity_uuid in self.counts.exits.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.exits_end = self.counts.total;
		self.counts.vending_start = self.counts.total;
		for entity_uuid in self.counts.vending.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.vending_end = self.counts.total;
		self.counts.conversations_start = self.counts.total;
		for entity_uuid in self.counts.conversations.iter() {
			self.uuid_to_index.insert(*entity_uuid, self.counts.total);
			self.index_to_uuid.insert(self.counts.total, *entity_uuid);
			self.counts.total = self.counts.total + 1;
		}
		self.counts.conversations_end = self.counts.total;
	}
}

fn main() {
	let mut builder: Builder = Builder{
		counts: Counts::new(),
		entities: HashMap::new(),
		main_file: MainFile::new(),
		uuid_to_index: HashMap::new(),
		index_to_uuid: HashMap::new(),
	};
	builder.main_file.begin();
	let mut conversations_file = ConversationsFile::new();
	conversations_file.begin();
	let mut vendings_file = VendingsFile::new();
	vendings_file.begin();
	let mut vending_item_ids: Vec<usize> = Vec::new();

	load_entities_from_dir(&mut builder, "../data");

	let mut conversation_index = 0;
	let mut vending_index = 0;

	builder.build_cache();

	for (uuid, entity) in builder.entities {
		let array_index = *builder.uuid_to_index.get(&uuid).unwrap();
		// all non-conversations and non-vending
		if array_index < builder.counts.vending_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.uuid_map.insert({}, {});\n",
					uuid,
					array_index,
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.uuids[{}] = {};\n",
					array_index,
					uuid
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.names[{}] = \"{}\";\n",
					array_index,
					str::replace(entity.name.clone().unwrap().as_str(), "\"", "\\\"")
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.descriptions[{}] = \"{}\";\n",
					array_index,
					str::replace(entity.description.unwrap().trim(), "\"", "\\\"")
				)
			);

			// non-locations
			if array_index >= builder.counts.items_start {

			}

			if array_index >= builder.counts.locations_start && array_index < builder.counts.locations_end {
				for item_slot in entity.items.unwrap() {
					builder.main_file.write_all(
						format!(
							"\tcomponents.location_items[{}].add({}, {});\n",
							array_index,
							item_slot.item_id,
							item_slot.quantity
						)
					);
				}
			}
		}

		// conversations only
		if array_index >= builder.counts.conversations_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.owns_conversation[{}] = Some({});\n",
					builder.uuid_to_index.get(&entity.speaker.unwrap()).unwrap(),
					builder.index_to_uuid.get(&conversation_index).unwrap(),
				)
			);

			conversations_file.open_root(uuid);
			for conversation in entity.prompts.unwrap() {
				conversations_file.render_conversation(
					&conversation,
					String::new()
				);
				builder.main_file.render_conversation(&conversation);
			}
			conversations_file.close_root();
			conversation_index = conversation_index + 1;
		}
		// vending only
		else if array_index >= builder.counts.vending_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.owns_vending[{}] = Some({});\n",
					builder.uuid_to_index.get(&entity.vendor.unwrap()).unwrap(),
					vending_index, // array_index,
				)
			);
			let vending = Vending {
				id: uuid,
				items: entity.vendables.unwrap()
			};
			vendings_file.render_vending(&vending);
			for item in vending.items {
				vending_item_ids.push(item.id);
			}
			vending_index = vending_index + 1;
		}
		// exits only
		else if array_index >= builder.counts.exits_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n",
					builder.uuid_to_index.get(&entity.location.unwrap()).unwrap(),
					builder.index_to_uuid.get(&array_index).unwrap(),
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.location_map[{}] = {};\n",
					array_index,
					entity.location.unwrap(),
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.destinations[{}] = {};\n",
					array_index - builder.counts.exits_start,
					entity.to.unwrap(),
				)
			);
		}
		// people only
		else if array_index >= builder.counts.people_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n",
					builder.uuid_to_index.get(&entity.location.unwrap()).unwrap(),
					builder.index_to_uuid.get(&array_index).unwrap()
				)
			);
			builder.main_file.write_all(
				format!(
					"\tcomponents.location_map[{}] = {};\n",
					array_index,
					entity.location.unwrap()
				)
			);
		}
		// items only
		else if array_index >= builder.counts.items_start {
			builder.main_file.write_all(
				format!(
					"\tcomponents.takeable[{}] = {};\n",
					array_index - builder.counts.items_start,
					&entity.takeable.unwrap()
				)
			);
		}

		builder.main_file.write_all(
			"\n".to_string()
		);
	}

	// inefficient
	let vending_location_index = builder.uuid_to_index.get(&builder.counts.vending_ether_uuid).unwrap();
	for vending_item_id in vending_item_ids {
		builder.main_file.write_all(
			format!(
				"\tcomponents.location_items[{}].add({}, 1);\n",
				vending_location_index,
				vending_item_id
			)
		);
	}

	conversations_file.end();
	vendings_file.end();
	builder.main_file.end(builder.counts.starting_location_uuid);
	write_components_file(&builder.counts, builder.counts.inventory_uuid);
}
