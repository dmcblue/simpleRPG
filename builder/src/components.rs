use std::fs::File;
use std::io::Write;

use super::counts::Counts;

pub fn write_components_file(counts: &Counts, inventory_id: usize /* not uuid */) {
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
		counts.total - counts.conversations_start, // conversations
		counts.total, // descriptions
		counts.exits.len(), // destinations
		counts.total - counts.conversations_start, // enabled
		counts.total, // location_map
		counts.locations.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.items.len(), // takeable
		counts.total, // uuids

		// Component init
		counts.total - counts.conversations_start, // conversations
		counts.total, // descriptions
		counts.exits.len(), // destinations
		counts.total - counts.conversations_start, // enabled
		counts.total, // location_map
		counts.locations.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.exits_start, // exists start
		counts.items_start, // items_start
		counts.people_start, // people_start
		inventory_id, // inventory_id
		counts.items.len(), // takeable
		counts.total, // uuids
	).as_bytes());
}
