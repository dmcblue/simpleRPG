mod components;
mod conversations;
mod counts;
mod vending;

// std
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::clone::Clone;

// ext
use serde::{Serialize, Deserialize};
use serde_yml;

// int
use components::{write_components_file};
use conversations::{ConversationNode, ConversationsFile};
use counts::Counts;
use vending::{Vending, VendingsFile, VendItem};

const ENTITY_TYPE_CONVERSATION: &str = "Conversation";
const ENTITY_TYPE_EXIT: &str = "Exit";
const ENTITY_TYPE_ITEM: &str = "Item";
const ENTITY_TYPE_LOCATION: &str = "Location";
const ENTITY_TYPE_PERSON: &str = "Person";
const ENTITY_TYPE_VENDING: &str = "Vending";



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Entity {
	#[serde(rename = "type")]
	entity_type: String,
	metaname: String,
	metadata: Vec<String>,
	id: Option<usize>,
	//#[serde(default)]
	name: Option<String>,
	description: Option<String>,
	// Location specific
	items: Option<Vec<usize>>,
	// Exit specific
	location: Option<usize>,
	takeable: Option<bool>,
	to: Option<usize>,
	// Vending specific
	vendables: Option<Vec<VendItem>>,
	vendor: Option<usize>,
	// Conversation specific
	speaker: Option<usize>,
	prompts: Option<Vec<ConversationNode>>,
}

fn main() {
	let paths = fs::read_dir("../data").unwrap();

	let mut file = File::create("../game/src/data/main.rs").unwrap();
	let _ = file.write_all(
		b"use super::components::Components;\n\
		// use super::vending::{Price, Vending, VendItem};\n\
		\n\
		pub fn load_data(components: &mut Components) {\n"
	);
	let mut conversations_file = ConversationsFile::new();
	conversations_file.begin();
	let mut vendings_file = VendingsFile::new();
	vendings_file.begin();
	let mut counts: Counts = Counts::new();
	let mut entities: HashMap<usize, Entity> = HashMap::new();

	for path in paths {
		let file_path = path.unwrap().path();
		let contents = fs::read_to_string(file_path).unwrap();
		let entity: Entity = serde_yml::from_str(&contents).unwrap();

		if entity.entity_type == "Game" {
			counts.starting_location_id = entity.location.unwrap();
		} else {
			let uuid: usize = entity.id.unwrap();
			match entity.entity_type.as_str() {
				ENTITY_TYPE_CONVERSATION => { counts.conversations.push(uuid); },
				ENTITY_TYPE_EXIT => { counts.exits.push(uuid); },
				ENTITY_TYPE_ITEM => { counts.items.push(uuid); },
				ENTITY_TYPE_LOCATION => { counts.locations.push(uuid); },
				ENTITY_TYPE_PERSON => { counts.people.push(uuid); },
				ENTITY_TYPE_VENDING => { counts.vending.push(uuid); },
				_ => ()
			}

			if entity.metaname == "Inventory" {
				counts.inventory_uuid = uuid;
			}
			entities.insert(uuid, entity);
		}
	}

	let mut uuid_to_index: HashMap<usize, usize> = HashMap::new();
	let mut conversation_index = 0;
	let mut vending_index = 0;

	counts.locations_start = counts.total;
	for entity_id in counts.locations.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.locations_end = counts.total;
	counts.items_start = counts.total;
	for entity_id in counts.items.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.items_end = counts.total;
	counts.people_start = counts.total;
	for entity_id in counts.people.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.people_end = counts.total;
	counts.exits_start = counts.total;
	for entity_id in counts.exits.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.exits_end = counts.total;
	counts.vending_start = counts.total;
	for entity_id in counts.vending.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.vending_end = counts.total;
	counts.conversations_start = counts.total;
	for entity_id in counts.conversations.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.conversations_end = counts.total;
	for (uuid, entity) in entities {
		let array_index = *uuid_to_index.get(&uuid).unwrap();
		// all non-conversations and non-vending
		if array_index < counts.vending_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.uuid_map.insert({}, {});\n",
					uuid,
					array_index,
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.uuids[{}] = {};\n",
					array_index,
					uuid
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.names[{}] = \"{}\";\n",
					array_index,
					str::replace(entity.name.clone().unwrap().as_str(), "\"", "\\\"")
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.descriptions[{}] = \"{}\";\n",
					array_index,
					str::replace(entity.description.clone().unwrap().as_str(), "\"", "\\\"")
				).as_bytes()
			);

			// non-locations
			if array_index >= counts.items_start {

			}

			if array_index >= counts.locations_start && array_index < counts.locations_end {
				let _ = file.write_all(
					format!(
						"\tcomponents.location_items[{}] = vec![{}];\n",
						array_index,
						entity.items.clone().unwrap().iter().
							map(|id| format!("{}", *uuid_to_index.get(&id).unwrap())).
							collect::<Vec<_>>().
							join(", ")
					).as_bytes()
				);
			}
		}

		// conversations only
		if array_index >= counts.conversations_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.owns_conversation[{}] = Some({});\n",
					uuid_to_index.get(&entity.speaker.unwrap()).unwrap(),
					conversation_index, // array_index,
				).as_bytes()
			);

			conversations_file.open_root(uuid);
			for conversation in entity.prompts.unwrap() {
				conversations_file.render_conversation(
					&conversation,
					String::new()
				);
			}
			conversations_file.close_root();
			conversation_index = conversation_index + 1;
		}
		// vending only
		else if array_index >= counts.vending_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.owns_vending[{}] = Some({});\n",
					uuid_to_index.get(&entity.vendor.unwrap()).unwrap(),
					vending_index, // array_index,
				).as_bytes()
			);
			let vending = Vending {
				id: uuid,
				items: entity.vendables.unwrap()
			};
			vendings_file.render_vending(&vending);
			vending_index = vending_index + 1;
		}
		// exits only
		else if array_index >= counts.exits_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n",
					uuid_to_index.get(&entity.location.unwrap()).unwrap(),
					array_index
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.location_map[{}] = {};\n",
					array_index,
					uuid_to_index.get(&entity.location.unwrap()).unwrap()
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.destinations[{}] = {};\n",
					array_index - counts.exits_start,
					uuid_to_index.get(&entity.to.unwrap()).unwrap()
				).as_bytes()
			);
		}
		// people only
		else if array_index >= counts.people_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n",
					uuid_to_index.get(&entity.location.unwrap()).unwrap(),
					array_index
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.location_map[{}] = {};\n",
					array_index,
					uuid_to_index.get(&entity.location.unwrap()).unwrap()
				).as_bytes()
			);
		}
		// items only
		else if array_index >= counts.items_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.takeable[{}] = {};\n",
					array_index - counts.items_start,
					&entity.takeable.unwrap()
				).as_bytes()
			);
		}

		let _ = file.write_all(
			"\n".as_bytes()
		);
	}
	conversations_file.end();
	vendings_file.end();

	let _ = file.write_all(
		format!(
			"}}\n\npub fn get_start_location_id() -> usize {{ {} }}",
			uuid_to_index.get(&counts.starting_location_id).unwrap()
		).as_bytes()
	);
	write_components_file(&counts, *uuid_to_index.get(&counts.inventory_uuid).unwrap());
}
