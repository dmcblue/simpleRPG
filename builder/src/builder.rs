// std
use std::fs::{
	File,
	metadata,
	read_dir,
	read_to_string
};
use std::collections::HashMap;

// ext

// int
use super::challenges::ChallengesFile;
use super::counts::Counts;
use super::entities::{
	Entity,
	ENTITY_TYPE_CHALLENGE_CARD,
	ENTITY_TYPE_CHALLENGE,
	ENTITY_TYPE_CHALLENGE_TYPE,
	ENTITY_TYPE_CONVERSATION,
	ENTITY_TYPE_EXIT,
	ENTITY_TYPE_ITEM,
	ENTITY_TYPE_LOCATION,
	ENTITY_TYPE_PERSON,
	ENTITY_TYPE_VENDING,
};
use super::main_file::MainFile;


pub struct Builder {
	pub challenges_file: ChallengesFile,
	pub counts: Counts,
	pub entities: HashMap<usize, Entity>,
	pub main_file: MainFile,
	pub uuid_to_index: HashMap<usize, usize>,
	pub index_to_uuid: HashMap<usize, usize>,
}

impl Builder {
	pub fn add_cache_item(&mut self, entity_uuid: usize) {
		self.uuid_to_index.insert(entity_uuid, self.counts.total);
		self.index_to_uuid.insert(self.counts.total, entity_uuid);
		self.counts.total = self.counts.total + 1;
	}

	pub fn build_cache(&mut self) {
		self.counts.locations.start = self.counts.total;
		for entity_uuid in self.counts.locations.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.locations.end = self.counts.total;

		self.counts.items.start = self.counts.total;
		for entity_uuid in self.counts.items.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.items.end = self.counts.total;

		self.counts.people.start = self.counts.total;
		for entity_uuid in self.counts.people.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.people.end = self.counts.total;

		self.counts.exits.start = self.counts.total;
		for entity_uuid in self.counts.exits.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.exits.end = self.counts.total;

		self.counts.vending.start = self.counts.total;
		for entity_uuid in self.counts.vending.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.vending.end = self.counts.total;

		self.counts.conversations.start = self.counts.total;
		for entity_uuid in self.counts.conversations.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.conversations.end = self.counts.total;

		self.counts.challenge_types.start = self.counts.total;
		for entity_uuid in self.counts.challenge_types.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.challenge_types.end = self.counts.total;

		self.counts.challenges.start = self.counts.total;
		for entity_uuid in self.counts.challenges.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.challenges.end = self.counts.total;

		self.counts.challenge_cards.start = self.counts.total;
		for entity_uuid in self.counts.challenge_cards.uuids.clone().iter() {
			self.add_cache_item(*entity_uuid);
		}
		self.counts.challenge_cards.end = self.counts.total;

	}

	pub fn get_entity(&self, uuid: usize) -> &Entity {
		self.entities.get(&uuid).unwrap()
	}

	pub fn load_entities_from_dir(&mut self, dir_name: &str) {
		let paths = read_dir(dir_name).unwrap();
		for path in paths {
			let file_path = path.unwrap().path();
			if !metadata(&file_path).unwrap().is_dir() {
				let contents = read_to_string(file_path).unwrap();
				let entity: Entity = serde_saphyr::from_str(&contents).unwrap();

				if entity.entity_type == "Game" {
					self.counts.starting_location_uuid = entity.location.unwrap();
				} else {
					log::info!("{:?}", entity);
					let uuid: usize = entity.id.unwrap();
					match entity.entity_type.as_str() {
						ENTITY_TYPE_CHALLENGE => { self.counts.challenges.uuids.push(uuid); },
						ENTITY_TYPE_CHALLENGE_CARD => { self.counts.challenge_cards.uuids.push(uuid); },
						ENTITY_TYPE_CHALLENGE_TYPE => {
							self.counts.challenge_types.uuids.push(uuid);
							self.challenges_file.process_challenge_type_attributes(&entity);
						},
						ENTITY_TYPE_CONVERSATION => { self.counts.conversations.uuids.push(uuid); },
						ENTITY_TYPE_EXIT => { self.counts.exits.uuids.push(uuid); },
						ENTITY_TYPE_ITEM => { self.counts.items.uuids.push(uuid); },
						ENTITY_TYPE_LOCATION => { self.counts.locations.uuids.push(uuid); },
						ENTITY_TYPE_PERSON => { self.counts.people.uuids.push(uuid); },
						ENTITY_TYPE_VENDING => { self.counts.vending.uuids.push(uuid); },
						_ => ()
					}

					if entity.metaname == "Inventory" {
						self.counts.inventory_uuid = uuid;
					} else if entity.metaname == "Vending Ether" {
						self.counts.vending_ether_uuid = uuid;
					}
					self.entities.insert(uuid, entity);
				}
			}
		}
	}
}
