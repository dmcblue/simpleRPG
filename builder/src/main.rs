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
mod components;
use components::{write_components_file};

mod conversations;
use conversations::{ConversationNode, ConversationsFile};

mod counts;
use counts::Counts;


const ENTITY_TYPE_CONVERSATION: &str = "Conversation";
const ENTITY_TYPE_EXIT: &str = "Exit";
const ENTITY_TYPE_ITEM: &str = "Item";
const ENTITY_TYPE_LOCATION: &str = "Location";
const ENTITY_TYPE_PERSON: &str = "Person";



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
	// Exit specific
	location: Option<usize>,
	takeable: Option<bool>,
	to: Option<usize>,
	// Conversation specific
	speaker: Option<usize>,
	prompts: Option<Vec<ConversationNode>>,
}

fn main() {
	let paths = fs::read_dir("../data").unwrap();

	let mut file = File::create("../game/src/data/main.rs").unwrap();
	let _ = file.write_all(
		b"use super::components::Components;\n\n\
		pub fn load_data(components: &mut Components) {\n"
	);
	let mut conversations_file = ConversationsFile::new();
	conversations_file.begin();
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
				_ => ()
			}
			if entity.metaname == "Inventory" {
				counts.inventory_uuid = uuid;
			}
			entities.insert(uuid, entity);
		}
	}

	let mut uuid_to_index: HashMap<usize, usize> = HashMap::new();

	for entity_id in counts.locations.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.items_start = counts.total;
	for entity_id in counts.items.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.people_start = counts.total;
	for entity_id in counts.people.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.exits_start = counts.total;
	for entity_id in counts.exits.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.conversations_start = counts.total;
	for entity_id in counts.conversations.iter() {
		uuid_to_index.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	for (uuid, entity) in entities {
		let array_index = *uuid_to_index.get(&uuid).unwrap();
		// all non-conversations
		if array_index < counts.conversations_start {
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
		}

		// conversations only
		if array_index >= counts.conversations_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.owns_conversation[{}] = {};\n",
					uuid_to_index.get(&entity.speaker.unwrap()).unwrap(),
					array_index,
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
		}
		// exits only
		else if array_index >= counts.exits_start {
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

	let _ = file.write_all(
		format!(
			"}}\n\npub fn get_start_location_id() -> usize {{ {} }}",
			uuid_to_index.get(&counts.starting_location_id).unwrap()
		).as_bytes()
	);
	write_components_file(&counts, *uuid_to_index.get(&counts.inventory_uuid).unwrap());
}
