// Builds in the Vending data files

// std
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::clone::Clone;

// ext
use serde::{Serialize, Deserialize};

// int
use super::entities::{Entity};

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct ChallengeType {
// 	pub id: usize,
// 	pub name: String,
// 	pub attributes: HashMap<String, usize>,
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct Challenge {
// 	pub id: usize,
// 	pub name: String,
// 	pub challenge_type_uuid: usize,
// 	pub level: usize,
// 	pub phases
// }

// pub struct ChallengePhase {
// 	pub id: usize,
// 	pub name: String,
// 	pub attributes: HashMap<usize, usize>,
// 	pub card_uuids: Vec<usize>,
// }

pub struct ChallengesFile {
	file_handle: File,
	type_attributes: HashMap<usize, HashMap<String, usize>>,
}

impl ChallengesFile {
	pub fn new() -> Self {
		Self {
			file_handle: File::create("../game/src/data/challenges_impl.rs").unwrap(),
			type_attributes: HashMap::new(),
		}
	}

	pub fn begin(&mut self) {
		let _ = self.file_handle.write_all(
			b"use std::collections::HashMap;\n\
			use super::challenges::{Challenge, ChallengeType, Phase, ChallengeCard, ChallengeEffect};\n\
			use super::components::{Components};\n\
			\n\
			pub fn load_challenges(components: &mut Components) {\n\
			\tcomponents.challenge_types = HashMap::new();\n\
			\tcomponents.challenges = HashMap::new();\n\
			\tcomponents.challenge_cards = HashMap::new();\n\n"
		);
	}

	pub fn end(&mut self) {
		let _ = self.file_handle.write_all(
			b"}\n"
		);
	}

	pub fn process_challenge_type_attributes(&mut self, challenge_type: &Entity) {
		self.type_attributes.insert(
			challenge_type.id.unwrap(),
			challenge_type.attributes.as_ref().unwrap().clone()
		);
	}

	pub fn render_card(&mut self, challenge_type: Entity) {

	}

	pub fn render_challenge_type(&mut self, challenge_type: Entity) {
		let _ = self.file_handle.write_all(
			format!(
				"\tcomponents.challenge_types.insert(\n\
				\t\t{},\n\
				\t\tChallengeType{{\n\
				\t\t\tname: String::from(\"{}\"),\n\
				\t\t\tattributes: HashMap::from([\n\
				",
				challenge_type.id.unwrap(),
				challenge_type.name.unwrap(),
			).as_bytes()
		);
		for (attribute_name, attribute_uuid) in challenge_type.attributes.unwrap() {
			let _ = self.file_handle.write_all(
				format!(
					"\t\t\t\t(String::from(\"{}\"), {}),\n",
					attribute_name,
					attribute_uuid,
				).as_bytes()
			);
		}
		let _ = self.file_handle.write_all(
			b"\t\t\t]),\n\
			\t\t}\n\
			\t);\n"
		);
	}

	pub fn render_challenge(&mut self, challenge: Entity) {
		let _ = self.file_handle.write_all(
			format!(
				"\tcomponents.challenges.insert(\n\
				\t\t{},\n\
				\t\tChallenge{{\n\
				\t\t\tchallenge_type_uuid: {},\n\
				\t\t\tname: String::from(\"{}\"),\n\
				\t\t\tlevel: {},\n\
				\t\t\tphases: vec![\n\
				",
				challenge.challenge_type.unwrap(),
				challenge.id.unwrap(),
				challenge.name.unwrap(),
				challenge.level.unwrap(),
			).as_bytes()
		);
		let type_attributes = self.type_attributes.get(&challenge.challenge_type.unwrap()).unwrap();
		for phase in challenge.phases.unwrap() {
			let attr_str: String =
				phase.attributes.
					iter().
					map(|(name, value)|
						format!("({}, {}),", type_attributes.get(name).unwrap(), value)
					).
					collect::<Vec<String>>().
					join("");
			let card_str: String =
				phase.cards.
					iter().
					map(|card_uuid| format!("{}", card_uuid) ).
					collect::<Vec<String>>().
					join("");
			let _ = self.file_handle.write_all(
				format!(
					"\t\t\t\tPhase {{\n\
					\t\t\t\t\tname: String::from(\"{}\"),\n\
					\t\t\t\t\tattributes: HashMap::from([{}]),\n\
					\t\t\t\t\tcards: vec![{}],\n\
					\t\t\t\t}},\n",
					phase.name,
					attr_str,
					card_str
				).as_bytes()
			);
		}
		let _ = self.file_handle.write_all(
			b"\t\t\t],\n\
			\t\t}\n\
			\t);\n"
		);
	}
}
// pub struct ChallengePhase {
// 	pub id: usize,
// 	pub name: String,
// 	pub attributes: Option<HashMap<String, usize>>,
// 	pub cards: Option<Vec<usize>>,
// }
// pub struct Phase {
// 	pub name: String,
// 	// uuid -> value
// 	pub attributes: HashMap<usize, usize>,
// 	// uuid
// 	pub cards: Vec<usize>,
// }


// pub struct Challenge {
// 	pub challenge_type: &ChallengeType,
// 	pub name: String,
// 	pub level: usize,
// 	pub phases: Vec<Phase>,
// }

// pub struct Phase {
// 	pub name: String,
// 	// uuid -> value
// 	pub attributes: HashMap<usize, usize>,
// 	// uuid
// 	pub cards: Vec<usize>,
// }

// #[derive(PartialEq, Clone, Copy, Debug)]
// pub enum ChallengeEffectType {
// 	TEMP_BUFF,
// }

// pub struct ChallengeCard {
// 	// pub id: usize,
// 	pub challenge_type: &ChallengeType,
// 	pub name: String,
// 	pub effects: Vec<ChallengeEffect>,
// }
