// std
use std::collections::HashMap;
use std::clone::{Clone};

// ext
use serde::{Serialize, Deserialize};

// int
use super::conversations::{
	ConversationNode,
	// ConversationsFile
};
use super::vending::{
	// Vending,
	// VendingsFile,
	VendItem
};

pub const ENTITY_TYPE_CHALLENGE_CARD: &str = "ChallengeCard";
pub const ENTITY_TYPE_CHALLENGE: &str = "Challenge";
pub const ENTITY_TYPE_CHALLENGE_TYPE: &str = "ChallengeType";
pub const ENTITY_TYPE_CONVERSATION: &str = "Conversation";
pub const ENTITY_TYPE_EXIT: &str = "Exit";
pub const ENTITY_TYPE_ITEM: &str = "Item";
pub const ENTITY_TYPE_LOCATION: &str = "Location";
pub const ENTITY_TYPE_PERSON: &str = "Person";
pub const ENTITY_TYPE_PLAYER_CARD: &str = "PlayerCard";
pub const ENTITY_TYPE_VENDING: &str = "Vending";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Effect {
	#[serde(rename = "type")]
	pub effect_type: String,
	pub arg_1: usize,
	pub arg_2: usize,
	pub arg_3: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemSlot {
	pub item_id: usize,
	pub quantity: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ChallengePhase {
	pub id: usize,
	pub name: String,
	pub attributes: HashMap<String, usize>,
	pub cards: Vec<usize>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Entity {
	#[serde(rename = "type")]
	pub entity_type: String,
	pub metaname: String,
	pub metadata: Vec<String>,
	pub id: Option<usize>,
	//#[serde(default)]
	pub name: Option<String>,
	pub description: Option<String>,
	// Location specific
	pub items: Option<Vec<ItemSlot>>,
	// Exit specific
	pub location: Option<usize>,
	pub takeable: Option<bool>,
	pub to: Option<usize>,
	// Vending specific
	pub vendables: Option<Vec<VendItem>>,
	pub vendor: Option<usize>,
	// Conversation specific
	pub speaker: Option<usize>,
	pub prompts: Option<Vec<ConversationNode>>,
	// ChallengeType
	pub attributes: Option<HashMap<String, usize>>,
	// Challenge
	pub challenge_type: Option<usize>, // and ChallengeCard
	pub level: Option<usize>,
	pub phases: Option<Vec<ChallengePhase>>,
	// Card specific
	pub effects: Option<Vec<Effect>>,
	pub starter: Option<usize>,
}
