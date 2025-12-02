use std::collections::HashMap;

use super::state::{ConversationPointer, State};
use super::data::{
	Components,
	ConversationNode,
	Event,
	EventType,
	Items,
	get_start_location_uuid
};
use super::game_mode::GameMode;
use super::scene::Scene;
use super::action::{Action, ActionType};

/*
	Game is meant to handle the actual game play
	not menus for saving, loading

	If we consolidate/abstract, then we need a game state
	or None even before the game is loaded.

	But what if we go the other direction. It's only handed a
	game state when its needed. Game is essentially a gamestate
	interpreter.
 */
pub struct Game<'a> {
	pub components: Components<'a>,
	pub mode: GameMode,
	pub scene: Scene,
	pub state: State,
}

impl Game<'_> {
	pub fn new() -> Self {
		Game {
			components: Components::new(),
			mode: GameMode::EXPLORE,
			scene: Scene {
				location_uuid: 0,
				entity_uuids: Vec::new(),
				actions: Vec::new(),
			},
			state: State {
				current_conversation: ConversationPointer {
					conversation_uuid: 0,
					path: Vec::new(),
				},
				current_location_uuid: get_start_location_uuid(),
				current_vending_uuid: 0,
				last_action_type: ActionType::GO,
				state_changes: HashMap::new()
			}
		}
	}

	pub fn get_conversation(&self) -> &ConversationNode {
		let mut pointer: &ConversationNode = 
			self.components.get_conversation(self.state.current_conversation.conversation_uuid);

		for index in &self.state.current_conversation.path {
			let mut i = index + 0;
			while !self.components.is_enabled(pointer.prompts[i].uuid) {	
				i = i + 1;
			}
			pointer = &pointer.prompts[i];
		}

		return pointer;
	}

	pub fn handle_action(&mut self, action: Action) {
		self.state.last_action_type = action.action_type.clone();
		match action.action_type {
			ActionType::CHECK_INVENTORY => (),
			ActionType::GO => {
				self.state.current_location_uuid =
					self.components.get_destination(action.arg_1.unwrap());
			},
			ActionType::LOOK => (),
			ActionType::TAKE => {
				let uuid = action.arg_1.unwrap();
				self.components.move_item_to(uuid, self.components.inventory_uuid);
				// record change to world state
				self.state.update_location(
					uuid,
					self.components.inventory_uuid
				);
			},
			ActionType::TALK => {
				let speaker_uuid = action.arg_1.unwrap();
				match self.components.get_conversation_by_speaker(speaker_uuid) {
					Some(converstaion_node) => {
						self.state.current_conversation.conversation_uuid = converstaion_node.uuid;
						self.state.current_conversation.path.clear();
						self.mode = GameMode::TALK;
					},
					None => {}
				}
			},
			ActionType::VEND => {
				let vendor_uuid = action.arg_1.unwrap();
				let vending = self.components.get_vending(vendor_uuid);
				self.state.current_vending_uuid = vending.uuid;
				self.mode = GameMode::VEND;
			},
		}

		// reset the scene so list of actions updates
		self.setup_scene();
	}

	pub fn handle_event(&mut self, event: Event) {
		match event.event_type {
			EventType::ENABLE_CONVERSATION => {
				self.components.set_enabled(event.arg_1.unwrap(),  true);
			}
		}
	}

	pub fn setup_scene(&mut self) {
		let entity_uuids: Vec<usize> = self.components.get_location(self.state.current_location_uuid).to_vec();
		self.scene.location_uuid = self.state.current_location_uuid;
		self.scene.entity_uuids = entity_uuids.clone();
		let exit_uuids: Vec<usize> = entity_uuids.clone(). //performance
				into_iter().
				filter(|uuid| self.components.is_exit(*uuid)).
				collect();
		let takeable_item_uuids: Vec<usize> =
			<Items as Clone>::clone(self.components.get_location_items(self.state.current_location_uuid)).
				into_iter().
				filter(|(uuid, quantity)| *quantity > 0 && self.components.is_takeable_item(*uuid)).
				map(|(uuid, _)| uuid ).
				collect::<Vec<_>>();
		let speaker_uuids: Vec<usize> = entity_uuids.clone(). //performance
				into_iter().
				filter(|id| self.components.is_speaker(*id)).
				collect();
		let vendor_uuids: Vec<usize> = entity_uuids.clone(). //performance
				into_iter().
				filter(|id| self.components.is_vendor(*id)).
				collect();
		self.scene.actions = Vec::new();

		// this is a waste of memory
		self.scene.actions.push(Action{action_type: ActionType::LOOK, ..Default::default()});
		for exit_uuid in &exit_uuids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::GO,
					arg_1: Some(*exit_uuid),
					..Default::default()
				}
			);
		}
		for speaker_uuid in &speaker_uuids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::TALK,
					arg_1: Some(*speaker_uuid),
					..Default::default()
				}
			);
		}

		for vendor_uuid in &vendor_uuids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::VEND,
					arg_1: Some(*vendor_uuid),
					..Default::default()
				}
			);
		}
		for item_uuid in &takeable_item_uuids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::TAKE,
					arg_1: Some(*item_uuid),
					arg_2: Some(1),
					..Default::default()
				}
			);
		}

		self.scene.actions.push(Action{action_type: ActionType::CHECK_INVENTORY, ..Default::default()});
	}
}
