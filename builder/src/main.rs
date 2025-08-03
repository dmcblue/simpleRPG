// use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
// use std::marker::Copy;
use std::clone::Clone;

use serde::{Serialize, Deserialize};
use serde_yml;
// use yaml_rust2::{Yaml, YamlLoader, YamlEmitter};

const ENTITY_TYPE_EXIT: &str = "Exit";
const ENTITY_TYPE_ITEM: &str = "Item";
const ENTITY_TYPE_LOCATION: &str = "Location";
const ENTITY_TYPE_PERSON: &str = "Person";

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Entity {
	#[serde(rename = "type")]
	entity_type: String,
	metaname: String,
	metadata: Vec<String>,
	starting_location: Option<usize>,
	id: Option<usize>,
	//#[serde(default)]
	name: Option<String>,
	description: Option<String>,
	// Exit Specific
	location: Option<usize>,
	takeable: Option<bool>,
	to: Option<usize>,
}

fn main() {
	let paths = fs::read_dir("../data").unwrap();

	let mut file = File::create("../game/src/data/main.rs").unwrap();
	let _ = file.write_all(
		b"use super::components::Components;\n\n\
		pub fn load_data(components: &mut Components) {\n"
	);
	let mut start_id: usize = 0;
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
			start_id = entity.starting_location.unwrap();
		} else {
			let id: usize = entity.id.unwrap();
			match entity.entity_type.as_str() {
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
	for (id, entity) in entities {
		let index = *id_map.get(&id).unwrap();
		let _ = file.write_all(
			format!(
				"\tcomponents.uuids[{}] = {};\n", 
				index, 
				id
			).as_bytes()
		);
		let _ = file.write_all(
			format!(
				"\tcomponents.names[{}] = \"{}\";\n", 
				index, 
				str::replace(entity.name.unwrap().as_str(), "\"", "\\\"")
			).as_bytes()
		);
		let _ = file.write_all(
			format!(
				"\tcomponents.descriptions[{}] = \"{}\";\n", 
				index, 
				str::replace(entity.description.unwrap().as_str(), "\"", "\\\"")
			).as_bytes()
		);
		// non-locations
		// if index >= people_start {
		// 	let _ = file.write_all(
		// 		format!(
		// 			"\tcomponents.locations[{}].push({});\n", 
		// 			index, 
		// 			id_map.get(&entity.location.unwrap()).unwrap()
		// 		).as_bytes()
		// 	);
		// }
		// exits only
		if index >= exits_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n", 
					id_map.get(&entity.location.unwrap()).unwrap(),
					index
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.destinations[{}] = {};\n", 
					index - exits_start, 
					id_map.get(&entity.to.unwrap()).unwrap()
				).as_bytes()
			);
		}
		// people only 
		else if index >= people_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n",
					id_map.get(&entity.starting_location.unwrap()).unwrap(),
					index
				).as_bytes()
			);
		}
		// items only 
		else if index >= items_start {
			let _ = file.write_all(
				format!(
					"\tcomponents.takeable[{}] = {};\n",
					index - items_start,
					&entity.takeable.unwrap()
				).as_bytes()
			);
			let _ = file.write_all(
				format!(
					"\tcomponents.locations[{}].push({});\n", 
					id_map.get(&entity.location.unwrap()).unwrap(),
					index
				).as_bytes()
			);
		}

		let _ = file.write_all(
			"\n".as_bytes()
		);
	}

	let _ = file.write_all(format!("}}\n\npub fn get_start_location_id() -> usize {{ {} }}", id_map.get(&start_id).unwrap()).as_bytes());
	let mut file = File::create("../game/src/data/components.rs").unwrap();
	let _ = file.write_all(format!("
use std::collections::HashMap;

pub struct Components<'a> {{
	pub descriptions: [&'a str; {}],
	pub destinations: [usize; {}],
	pub locations: [Vec<usize>; {}],
	pub names: [&'a str; {}],
	pub exits_start: usize,
	pub items_start: usize,
	pub people_start: usize,
	pub inventory_id: usize,
	pub takeable: [bool; {}],
	pub uuids: [usize; {}],
}}

pub fn make_components<'a>() -> Components<'a> {{
	return Components {{ 
		descriptions: [\"\"; {}],
		destinations: [0; {}],
		locations: [(); {}].map(|_| Vec::new()),
		names: [\"\"; {}],
		exits_start: {},
		items_start: {},
		people_start: {},
		inventory_id: {},
		takeable: [false; {}],
		uuids: [0; {}],
	}};
}}
", 
		// Component Struct Definition
		index,
		exits.len(),
		locations.len(),
		index,
		items.len(),
		index,

		// Component init
		index,
		exits.len(),
		locations.len(),
		index,
		exits_start,
		items_start,
		people_start,
		id_map.get(&inventory_id).unwrap(),
		items.len(),
		index,
	).as_bytes());
}
