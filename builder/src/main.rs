use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::clone::Clone;
// use std::collections::VecDeque;

use serde::{Serialize, Deserialize};
use serde_yml;

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
	let mut start_id: usize = 0;
	let mut conversations: Vec<usize> = Vec::new();
	let mut exits: Vec<usize> = Vec::new();
	let mut items: Vec<usize> = Vec::new();
	let mut locations: Vec<usize> = Vec::new();
	let mut people: Vec<usize> = Vec::new();
	let mut entities: HashMap<usize, Entity> = HashMap::new();
	let mut inventory_id: usize = 0;
	for path in paths {
		let file_path = path.unwrap().path();
		let contents = fs::read_to_string(file_path).unwrap();
		let entity: Entity = serde_yml::from_str(&contents).unwrap();

		if entity.entity_type == "Game" {
			start_id = entity.location.unwrap();
		} else {
			let id: usize = entity.id.unwrap();
			match entity.entity_type.as_str() {
				ENTITY_TYPE_CONVERSATION => { conversations.push(id); },
				ENTITY_TYPE_EXIT => { exits.push(id); },
				ENTITY_TYPE_ITEM => { items.push(id); },
				ENTITY_TYPE_LOCATION => { locations.push(id); },
				ENTITY_TYPE_PERSON => { people.push(id); },
				_ => ()
			}
			if entity.metaname == "Inventory" {
				inventory_id = id;
			}
			entities.insert(id, entity);
		}
	}
	let mut index: usize = 0;
	let mut id_map: HashMap<usize, usize> = HashMap::new();

	for entity_id in locations.iter() {
		id_map.insert(*entity_id, index);
		index = index + 1;
	}
	let items_start = index;
	for entity_id in items.iter() {
		id_map.insert(*entity_id, index);
		index = index + 1;
	}
	let people_start = index;
	for entity_id in people.iter() {
		id_map.insert(*entity_id, index);
		index = index + 1;
	}
	let exits_start = index;
	for entity_id in exits.iter() {
		id_map.insert(*entity_id, index);
		index = index + 1;
	}
	let conversations_start = index;
	for entity_id in conversations.iter() {
		id_map.insert(*entity_id, index);
		index = index + 1;
	}
	for (id, entity) in entities {
		let array_index = *id_map.get(&id).unwrap();
		// all non-conversations
		if array_index < conversations_start {
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
			if array_index >= items_start {
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
		if array_index >= conversations_start {
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
			let mut stack: Vec<ConversationNode> = Vec::new();
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
		else if array_index >= exits_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.destinations[{}] = {};\n",
					array_index - exits_start,
					id_map.get(&entity.to.unwrap()).unwrap()
				).as_bytes()
			);
		}
		// people only
		else if array_index >= people_start {

		}
		// items only
		else if array_index >= items_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.takeable[{}] = {};\n",
					array_index - items_start,
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

	let _ = file.write_all(format!("}}\n\npub fn get_start_location_id() -> usize {{ {} }}", id_map.get(&start_id).unwrap()).as_bytes());
	let mut file = File::create("../game/src/data/components.rs").unwrap();
	let _ = file.write_all(format!("
use super::conversations::{{ConversationRoot, ConversationNode}};

pub struct Components<'a> {{
	pub conversations: [ConversationRoot; {}],
	pub descriptions: [&'a str; {}],
	pub destinations: [usize; {}],
	pub enabled: [bool; {}],
	pub location_map: [usize; {}],
	pub locations: [Vec<usize>; {}],
	pub names: [&'a str; {}],
	pub owns_conversation: [usize; {}],
	pub exits_start: usize,
	pub items_start: usize,
	pub people_start: usize,
	pub inventory_id: usize,
	pub takeable: [bool; {}],
	pub uuids: [usize; {}],
}}

pub fn make_components<'a>() -> Components<'a> {{
	return Components {{
		conversations: [ConversationRoot::new(); {}],
		descriptions: [\"\"; {}],
		destinations: [0; {}],
		enabled: [false; {}],
		location_map: [0; {}],
		locations: [(); {}].map(|_| Vec::new()),
		names: [\"\"; {}],
		owns_conversation: [0; {}],
		exits_start: {},
		items_start: {},
		people_start: {},
		inventory_id: {},
		takeable: [false; {}],
		uuids: [0; {}],
	}};
}}

impl Components<'_> {{
	pub fn move_to(&mut self, entity_uuid: usize, new_location_id: usize) {{
		let starting_location_id = self.location_map[entity_uuid];
		let index = self.locations[starting_location_id].iter().position(|eid| *eid == entity_uuid).unwrap();
		self.locations[starting_location_id].remove(index);
		self.location_map[entity_uuid] = new_location_id;
		self.locations[new_location_id].push(entity_uuid);
	}}
}}
",
		// Component Struct Definition
		index - conversations_start, // conversations
		index, // descriptions
		exits.len(), // destinations
		index - conversations_start, // enabled
		index, // location_map
		locations.len(), // locations
		index, // names
		index, // owns_conversation
		items.len(), // takeable
		index, // uuids

		// Component init
		index - conversations_start, // conversations
		index, // descriptions
		exits.len(), // destinations
		index - conversations_start, // enabled
		index, // location_map
		locations.len(), // locations
		index, // names
		index, // owns_conversation
		exits_start, // exists start
		items_start, // items_start
		people_start, // people_start
		id_map.get(&inventory_id).unwrap(), // intentory_id
		items.len(), // takeable
		index, // uuids
	).as_bytes());
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
