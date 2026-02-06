// Creates the main Components struct with
// the correct array sizes

use std::fs::File;
use std::io::Write;

use super::counts::Counts;

pub fn write_components_file(counts: &Counts, inventory_uuid: usize /* not uuid */) {
	let mut file = File::create("../game/src/data/components.rs").unwrap();
	let _ = file.write_all(format!("
use std::collections::HashMap;

use super::challenges::{{Challenge, ChallengeCard, ChallengeType}};
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
	pub challenge_cards: HashMap<usize, ChallengeCard>,
	pub challenge_types: HashMap<usize, ChallengeType>,
	pub challenges: HashMap<usize, Challenge>,
	pub conversations: [ConversationNode; {}],
	pub conversations_start: usize,
	pub current_player_card_uuids: Vec<usize>,
	pub descriptions: [&'a str; {}],
	pub destinations: [usize; {}],
	pub enabled: HashMap<usize, bool>,
	pub location_items: [Items; {}],
	// entity index => location uuid
	pub location_map: [usize; {}],
	pub locations: [Vec<usize>; {}],
	pub names: [&'a str; {}],
	pub owns_conversation: [Option<usize>; {}], // entity_index => conversation_index (ie not entity)
	pub owns_vending: [Option<usize>; {}],
	pub exits_start: usize,
	pub items_start: usize,
	pub people_start: usize,
	pub player_cards: HashMap<usize, ChallengeCard>,
	pub inventory_uuid: usize,
	pub takeable: [bool; {}],
	pub uuid_map: HashMap<usize, usize>,
	pub uuids: [usize; {}],
	pub vendings: HashMap<usize, Vending>,
}}

impl Components<'_> {{
	pub fn new() -> Self {{
		return Components {{
			challenge_cards: HashMap::new(),
			challenge_types: HashMap::new(),
			challenges: HashMap::new(),
			conversations: [(); {}].map(|_| ConversationNode::new()),
			conversations_start: {},
			current_player_card_uuids: Vec::new(),
			descriptions: [\"\"; {}],
			destinations: [0; {}],
			enabled: HashMap::new(),
			location_items: [(); {}].map(|_| Items::new()),
			location_map: [0; {}],
			locations: [(); {}].map(|_| Vec::new()),
			names: [\"\"; {}],
			owns_conversation: [None; {}],
			owns_vending: [None; {}],
			exits_start: {},
			items_start: {},
			people_start: {},
			player_cards: HashMap::new(),
			inventory_uuid: {},
			takeable: [false; {}],
			uuid_map: HashMap::new(),
			uuids: [0; {}],
			vendings: HashMap::new(),
		}};
	}}
}}
",
		// Component Struct Definition
		counts.conversations.end - counts.conversations.start, // conversations
		counts.total, // descriptions
		counts.exits.uuids.len(), // destinations
		counts.locations.uuids.len(), // location items
		counts.total, // location_map
		counts.locations.uuids.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.total, // owns_vending
		counts.items.uuids.len(), // takeable
		counts.total, // uuids

		// Component init
		counts.conversations.end - counts.conversations.start, // conversations
		counts.conversations.start, // conversations_start
		counts.total, // descriptions
		counts.exits.uuids.len(), // destinations
		counts.locations.uuids.len(), // location items
		counts.total, // location_map
		counts.locations.uuids.len(), // locations
		counts.total, // names
		counts.total, // owns_conversation
		counts.total, // owns_vending
		counts.exits.start, // exists start
		counts.items.start, // items_start
		counts.people.start, // people_start
		inventory_uuid, // inventory_id
		counts.items.uuids.len(), // takeable
		counts.total, // uuids
		// counts.vending.len(), // vendings
	).as_bytes());
}
