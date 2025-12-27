mod builder;
mod challenges;
mod components;
mod conversations;
mod counts;
mod entities;
mod main_file;
mod vending;

// std
use std::collections::HashMap;
use std::clone::Clone;

// ext
use log4rs;

// int
use builder::{
	Builder,
};
use challenges::{ChallengesFile};
use components::{write_components_file};
use conversations::{
	ConversationsFile
};
use counts::Counts;
use main_file::MainFile;
use vending::{
	Vending,
	VendingsFile,
	// VendItem
};





fn main() {
	log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
	log::info!("Starting up");
	let mut builder: Builder = Builder{
		challenges_file: ChallengesFile::new(),
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
	builder.challenges_file.begin();

	let dirs = [
		"challenges",
		"challenge_types",
		"conversations",
		"exits",
		"general",
		"items",
		"locations",
		"persons",
		"vending",
	];
	for dir in dirs {
		builder.load_entities_from_dir(format!("../data/{}", dir).as_str());
	}
	// load_entities_from_dir(&mut builder, "../data");

	let mut conversation_index = 0;
	let mut vending_index = 0;

	builder.build_cache();

	for (uuid, entity) in builder.entities {
		log::info!("{}, {:?}", uuid, entity);
		let array_index = *builder.uuid_to_index.get(&uuid).unwrap();
		// all non-conversations and non-vending
		if array_index < builder.counts.vending.start {
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
					str::replace(entity.description.clone().unwrap().trim(), "\"", "\\\"")
				)
			);

			// non-locations
			if array_index >= builder.counts.items.start {

			}

			if array_index >= builder.counts.locations.start && array_index < builder.counts.locations.end {
				for item_slot in entity.items.clone().unwrap() {
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

		// match builder.counts.in_range_of(array_index) {
		// 	"bobby" => (),
		// 	_ => ()
		// }
		// challenges only
		// if array_index >= builder.counts.challenges.start {
		if builder.counts.cards.in_range(array_index) {

		}
		else if builder.counts.challenges.in_range(array_index) {
			builder.challenges_file.render_challenge(entity);
		}
		// challenge types only
		// else if array_index >= builder.counts.challenge_types.start {
		else if builder.counts.challenge_types.in_range(array_index) {
			builder.challenges_file.render_challenge_type(entity);
		}
		// conversations only
		// else if array_index >= builder.counts.conversations.start {
		else if builder.counts.conversations.in_range(array_index) {
			builder.main_file.write_all(
				format!(
					"\tcomponents.owns_conversation[{}] = Some({});\n",
					builder.uuid_to_index.get(&entity.speaker.unwrap()).unwrap(),
					builder.index_to_uuid.get(&conversation_index).unwrap(),
				)
			);

			conversations_file.open_root(uuid);
			for conversation in entity.prompts.clone().unwrap() {
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
		// else if array_index >= builder.counts.vending.start {
		else if builder.counts.vending.in_range(array_index) {
			builder.main_file.write_all(
				format!(
					"\tcomponents.owns_vending[{}] = Some({});\n",
					builder.uuid_to_index.get(&entity.vendor.unwrap()).unwrap(),
					vending_index, // array_index,
				)
			);
			let vending = Vending {
				id: uuid,
				items: entity.vendables.clone().unwrap()
			};
			vendings_file.render_vending(&vending);
			for item in vending.items {
				vending_item_ids.push(item.id);
			}
			vending_index = vending_index + 1;
		}
		// exits only
		// else if array_index >= builder.counts.exits.start {
		else if builder.counts.exits.in_range(array_index) {
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
					array_index - builder.counts.exits.start,
					entity.to.unwrap(),
				)
			);
		}
		// people only
		// else if array_index >= builder.counts.people.start {
		else if builder.counts.people.in_range(array_index) {
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
		// else if array_index >= builder.counts.items.start {
		else if builder.counts.items.in_range(array_index) {
			builder.main_file.write_all(
				format!(
					"\tcomponents.takeable[{}] = {};\n",
					array_index - builder.counts.items.start,
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

	builder.challenges_file.end();
	conversations_file.end();
	vendings_file.end();
	builder.main_file.end(builder.counts.starting_location_uuid);
	write_components_file(&builder.counts, builder.counts.inventory_uuid);
}
