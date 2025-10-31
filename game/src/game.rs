use std::collections::HashMap;

use super::state::{ConversationPointer, State};
use super::data::{
	Components,
	ConversationNode,
	Items,
	get_start_location_id
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
				location_id: 0,
				entity_ids: Vec::new(),
				actions: Vec::new(),
			},
			state: State {
				current_conversation: ConversationPointer {
					conversation_id: 0,
					path: Vec::new(),
				},
				current_location_id: get_start_location_id(),
				current_vending_index: 0,
				last_action_type: ActionType::GO,
				state_changes: HashMap::new()
			}
		}
	}

	pub fn get_conversation(&self) -> &ConversationNode {
		let mut pointer: &ConversationNode = &self.components.conversations[self.state.current_conversation.conversation_id];
		for index in &self.state.current_conversation.path {
			let mut i = index + 0;
			while !pointer.prompts[i].enabled {
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
				self.state.current_location_id =
					self.components.destinations[action.arg_1.unwrap() - self.components.exits_start]
			},
			ActionType::LOOK => (),
			ActionType::TAKE => {
				let id = action.arg_1.unwrap();
				self.components.move_item_to(self.components.uuids[id], self.components.inventory_id);
				// record change to world state
				self.state.update_location(
					self.components.uuids[id],
					self.components.uuids[self.components.inventory_id]
				);
			},
			ActionType::TALK => {
				let speaker_id = action.arg_1.unwrap();
				match self.components.owns_conversation[speaker_id] {
					Some(conversation_id) => {
						self.state.current_conversation.conversation_id = conversation_id;
						self.state.current_conversation.path.clear();
						self.mode = GameMode::TALK;
					},
					None => {
						log::info!("Oh my gosh no");
					}
				}
			},
			ActionType::VEND => {
				let vendor_id = action.arg_1.unwrap();
				match self.components.owns_vending[vendor_id] {
					Some(vending_id) => {
						// DAN HERE VEND
						self.state.current_vending_index = vending_id;
						self.mode = GameMode::VEND;
					},
					None => {
						log::info!("Oh my gosh no");
					}
				}
			},
		}

		// reset the scene so list of actions updates
		self.setup_scene();
	}

	pub fn setup_scene(&mut self) {
		let entity_ids: Vec<usize> = self.components.locations[self.state.current_location_id].to_vec();
		self.scene.location_id = self.state.current_location_id;
		self.scene.entity_ids = entity_ids.clone();
		let exit_ids: Vec<usize> = entity_ids.clone(). //performance
				into_iter().
				filter(|id| self.components.is_exit(*id)).
				collect();
		let takeable_item_ids: Vec<usize> =
			<Items as Clone>::clone(&self.components.location_items[self.state.current_location_id]).
				into_iter().
				filter(|(id, quantity)| *quantity > 0 ).
				map(|(id, _)| self.components.get_array_id(&id) ).
				filter(|id| self.components.is_takeable_item(*id) ).
				collect::<Vec<_>>();
		let speaker_ids: Vec<usize> = entity_ids.clone(). //performance
				into_iter().
				filter(|id| self.components.is_speaker(*id)).
				collect();
		let vendor_ids: Vec<usize> = entity_ids.clone(). //performance
				into_iter().
				filter(|id| self.components.is_vendor(*id)).
				collect();
		self.scene.actions = Vec::new();

		// this is a waste of memory
		self.scene.actions.push(Action{action_type: ActionType::LOOK, ..Default::default()});
		for exit_id in &exit_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::GO,
					arg_1: Some(*exit_id),
					..Default::default()
				}
			);
		}
		for speaker_id in &speaker_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::TALK,
					arg_1: Some(*speaker_id),
					..Default::default()
				}
			);
		}

		for vendor_id in &vendor_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::VEND,
					arg_1: Some(*vendor_id),
					..Default::default()
				}
			);
		}
		for item_id in &takeable_item_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::TAKE,
					arg_1: Some(*item_id),
					..Default::default()
				}
			);
		}
		self.scene.actions.push(Action{action_type: ActionType::CHECK_INVENTORY, ..Default::default()});
	}
}
