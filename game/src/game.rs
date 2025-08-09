use super::state::State;
use super::data::Components;
use super::scene::Scene;
use super::action::{Action, ActionType};

pub struct Game<'a> {
	pub state: State,
	pub components: Components<'a>,
	pub scene: Scene,
}

impl Game<'_> {
	pub fn setup_scene(&mut self) {
		let entity_ids: Vec<usize> = self.components.locations[self.state.current_location].to_vec();
		self.scene.location_id = self.state.current_location;
		self.scene.entity_ids = entity_ids.clone();
		self.scene.exit_ids = entity_ids.clone(). //performance
				into_iter().
				filter(|id| {
					if *id < self.components.exits_start {
						return false;
					}
					self.components.destinations.contains(&(*id - self.components.exits_start))
				}).
				collect();
		self.scene.takeable_item_ids = entity_ids.clone(). //performance
				into_iter().
				filter(|id| {
					if *id < self.components.items_start || *id >= self.components.people_start {
						return false;
					}
					self.components.takeable[*id - self.components.items_start]
				}).
				collect();
		self.scene.actions = Vec::new();

		// this is a waste of memory
		self.scene.actions.push(Action{action_type: ActionType::LOOK, ..Default::default()});
		for exit_id in &self.scene.exit_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::GO,
					arg_1: Some(*exit_id),
					..Default::default()
				}
			);
		}
		for item_id in &self.scene.takeable_item_ids {
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

	pub fn handle_action(&mut self, action: Action) {
		self.state.last_action_type = action.action_type.clone();
		match action.action_type {
			ActionType::CHECK_INVENTORY => (),
			ActionType::GO => {
				self.state.current_location =
					self.components.destinations[action.arg_1.unwrap() - self.components.exits_start]
			},
			ActionType::LOOK => (),
			ActionType::TAKE => {
				let id = action.arg_1.unwrap();
				let index = self.components.locations[self.state.current_location].iter().position(|eid| *eid == id).unwrap();
				self.components.locations[self.state.current_location].remove(index);
				self.components.locations[self.components.inventory_id].push(id);
				self.components.location_map[self.components.inventory_id] = id;
				// record change to world state
				self.state.update_location(self.components.uuids[id], self.components.inventory_id);
			},
			ActionType::TALK => ()
		}

		// reset the scene so list of actions updates
		self.setup_scene();
	}
}
