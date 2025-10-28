// Creates the main Components struct with
// the correct array sizes

use std::fs::File;
use std::io::Write;

use super::counts::Counts;

pub fn write_components_file(counts: &Counts, inventory_id: usize /* not uuid */) {
	let mut file = File::create("../game/src/data/components.rs").unwrap();
	let _ = file.write_all(format!("
use std::collections::HashMap;

use super::conversations::{{ConversationNode}};
use super::items::{{Items}};
use super::vending::{{Price, Vending, VendItem}};

// Order in array:
// - locations
// - items
// - people
// - exits
// - conversations

pub struct Components<'a> {{
	pub conversations: [ConversationNode; {}],
	pub descriptions: [&'a str; {}],
	pub destinations: [usize; {}],
	pub enabled: [bool; {}],
	pub location_items: [Items; {}],
	pub location_map: [usize; {}],
	pub locations: [Vec<usize>; {}],
	pub names: [&'a str; {}],
	pub owns_conversation: [Option<usize>; {}],
	pub owns_vending: [Option<usize>; {}],
	pub exits_start: usize,
	pub items_start: usize,
	pub people_start: usize,
	pub inventory_id: usize,
	pub takeable: [bool; {}],
	pub uuid_map: HashMap<usize, usize>,
	pub uuids: [usize; {}],
	pub vendings: [Vending; {}],
}}

impl Components<'_> {{
	pub fn new() -> Self {{
		return Components {{
			conversations: [(); {}].map(|_| ConversationNode::new()),
			descriptions: [\"\"; {}],
			destinations: [0; {}],
			enabled: [false; {}],
			location_items: [(); {}].map(|_| Items::new()),
			location_map: [0; {}],
			locations: [(); {}].map(|_| Vec::new()),
			names: [\"\"; {}],
			owns_conversation: [None; {}],
			owns_vending: [None; {}],
			exits_start: {},
			items_start: {},
			people_start: {},
			inventory_id: {},
			takeable: [false; {}],
			uuid_map: HashMap::new(),
			uuids: [0; {}],
			vendings: [(); {}].map(|_| Vending::new()),
		}};
	}}
}}
",
		// Component Struct Definition
		counts.total - counts.conversations_start, // conversations
		counts.total, // descriptions
		counts.exits.len(), // destinations
		counts.total - counts.conversations_start, // enabled
		counts.locations.len(), // location items
		counts.total, // location_map
		counts.locations.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.total, // owns_vending
		counts.items.len(), // takeable
		counts.total, // uuids
		counts.vending.len(), // vendings

		// Component init
		counts.total - counts.conversations_start, // conversations
		counts.total, // descriptions
		counts.exits.len(), // destinations
		counts.total - counts.conversations_start, // enabled
		counts.locations.len(), // location items
		counts.total, // location_map
		counts.locations.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.total, // owns_vending
		counts.exits_start, // exists start
		counts.items_start, // items_start
		counts.people_start, // people_start
		inventory_id, // inventory_id
		counts.items.len(), // takeable
		counts.total, // uuids
		counts.vending.len(), // vendings
	).as_bytes());
}
