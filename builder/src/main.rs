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

mod counts;
use counts::Counts;


const ENTITY_TYPE_CONVERSATION: &str = "Conversation";
const ENTITY_TYPE_EXIT: &str = "Exit";
const ENTITY_TYPE_ITEM: &str = "Item";
const ENTITY_TYPE_LOCATION: &str = "Location";
const ENTITY_TYPE_PERSON: &str = "Person";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct ConversationNode {
	id: usize,
	prompt: String,
	response: String,
	after: Option<String>,
	enabled: bool,
	prompts: Vec<ConversationNode>,
}

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
	let mut conversations_file = File::create("../game/src/data/conversations.rs").unwrap();
	let _ = conversations_file.write_all(
		b"use super::components::Components;\n\n\
		pub struct ConversationRoot {\n\
		\tid: usize,\n\
		\tprompts: Vec<ConversationNode>,
		}\n\n\
		pub struct ConversationNode {\n\
		\tid: usize,\n\
		\tprompt: String,\n\
		\tresponse: String,\n\
		\tprompts: Vec<ConversationNode>,\
		}\n\n\
		impl ConversationRoot {\n\
		\tpub fn new () -> Self {\n\
		\t\treturn ConversationRoot {\n\
		\t\t\tid: 0,\n\
		\t\t\tprompts: Vec::new(),\n\
		\t\t};\n\
		\t}\n\
		}\n\n\
		pub fn load_conversations(components: &mut Components) {\n\
		\tcomponents.conversations = [\n"
	);
	let mut counts: Counts = Counts::new();
	let mut entities: HashMap<usize, Entity> = HashMap::new();

	for path in paths {
		let file_path = path.unwrap().path();
		let contents = fs::read_to_string(file_path).unwrap();
		let entity: Entity = serde_yml::from_str(&contents).unwrap();

		if entity.entity_type == "Game" {
			counts.starting_location_id = entity.location.unwrap();
		} else {
			let id: usize = entity.id.unwrap();
			match entity.entity_type.as_str() {
				ENTITY_TYPE_CONVERSATION => { counts.conversations.push(id); },
				ENTITY_TYPE_EXIT => { counts.exits.push(id); },
				ENTITY_TYPE_ITEM => { counts.items.push(id); },
				ENTITY_TYPE_LOCATION => { counts.locations.push(id); },
				ENTITY_TYPE_PERSON => { counts.people.push(id); },
				_ => ()
			}
			if entity.metaname == "Inventory" {
				counts.inventory_uuid = id;
			}
			entities.insert(id, entity);
		}
	}

	// uuid to array id
	let mut id_map: HashMap<usize, usize> = HashMap::new();

	for entity_id in counts.locations.iter() {
		id_map.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.items_start = counts.total;
	for entity_id in counts.items.iter() {
		id_map.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.people_start = counts.total;
	for entity_id in counts.people.iter() {
		id_map.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.exits_start = counts.total;
	for entity_id in counts.exits.iter() {
		id_map.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	counts.conversations_start = counts.total;
	for entity_id in counts.conversations.iter() {
		id_map.insert(*entity_id, counts.total);
		counts.total = counts.total + 1;
	}
	for (id, entity) in entities {
		let array_index = *id_map.get(&id).unwrap();
		// all non-conversations
		if array_index < counts.conversations_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.uuids[{}] = {};\n",
					array_index,
					id
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
						id_map.get(&entity.location.unwrap()).unwrap(),
						array_index
					).as_bytes()
				);
				let _ = file.write_all(
					format!(
						"\tcomponents.location_map[{}] = {};\n",
						array_index,
						id_map.get(&entity.location.unwrap()).unwrap()
					).as_bytes()
				);
			}
		}

		// conversations only
		if array_index >= counts.conversations_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.owns_conversation[{}] = {};\n",
					id_map.get(&entity.speaker.unwrap()).unwrap(),
					array_index,
				).as_bytes()
			);
			let _ = conversations_file.write_all(
				format!(
					"\t\tConversationRoot{{\n\
					\t\t\tid: {},\n\
					\t\t\tprompts: vec![\n\
					",
					id
				).as_bytes()
			);
			// let mut stack: Vec<ConversationNode> = Vec::new();
			for conversation in entity.prompts.unwrap() {
				// needs to be iterative to not be awful
				render_conversation(
					&mut conversations_file,
					&conversation,
					String::from("\t")
				);
			}
			let _ = conversations_file.write_all(
				b"\t\t\t]\n\t\t},\n"
			);
		}
		// exits only
		else if array_index >= counts.exits_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.destinations[{}] = {};\n",
					array_index - counts.exits_start,
					id_map.get(&entity.to.unwrap()).unwrap()
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
	let _ = conversations_file.write_all(
		b"\t];\n}\n"
	);

	let _ = file.write_all(
		format!(
			"}}\n\npub fn get_start_location_id() -> usize {{ {} }}",
			id_map.get(&counts.starting_location_id).unwrap()
		).as_bytes()
	);
	write_components_file(&counts, *id_map.get(&counts.inventory_uuid).unwrap());
}

fn render_conversation(
	conversations_file: &mut File,
	conversation: &ConversationNode,
	depth: String
) {
	let _ = conversations_file.write_all(
		format!(
			"{}\t\tConversationNode{{\n\
			{}\t\t\tid: {},\n\
			{}\t\t\tprompt: String::from(\"{}\"),\n\
			{}\t\t\tresponse: String::from(\"{}\"),\n\
			{}\t\t\tprompts: vec![\n",
			depth,
			depth,
			conversation.id,
			depth,
			conversation.prompt,
			depth,
			conversation.response,
			depth,
		).as_bytes()
	);
	for c in conversation.prompts.clone() {
		render_conversation(
			conversations_file,
			&c,
			format!("\t{}", depth)
		);
	}
	let _ = conversations_file.write_all(
		format!(
			"{}\t\t\t]\n\
			{}\t\t}}\n",
			depth,
			depth
		).as_bytes()
	);
}
