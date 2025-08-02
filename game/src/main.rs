use std::io::{self, Write};
use simple::{Rect, Window};
use macroquad::prelude::*;

mod data;

#[derive(PartialEq, Clone)]
enum ActionType {
	CHECK_INVENTORY,
	GO,
	LOOK,
	TAKE,
	TALK
}
// #[derive(Default)]
struct Action {
	action_type: ActionType,
	arg_1: Option<usize>,
	// arg_2: Option<usize>,
	// arg_3: Option<usize>,
}
// #[derive(Default)]
struct Scene {
	location_id: usize,
	entity_ids: Vec<usize>,
	exit_ids: Vec<usize>,
	takeable_item_ids: Vec<usize>,
	actions: Vec<Action>,
}
impl Default for Action {
	fn default() -> Action {
		Action {
			action_type: ActionType::LOOK,
			arg_1: None,
			// arg_2: None,
			// arg_3: None,
		}
	}
}

struct State {
	pub current_location: usize,
	pub last_action_type: ActionType,
}

struct Game<'a> {
	state: State,
	components: data::Components<'a>,
	scene: Scene
}

impl Game<'_> {
	fn setup_scene(&mut self) {
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

	fn render(&self) {
		if self.state.last_action_type == ActionType::GO {
			println!("You are at {}", self.components.names[self.scene.location_id]);
			println!("You see {}", self.components.descriptions[self.scene.location_id]);
			for entity_id in &self.scene.entity_ids {
				println!("{}", self.components.names[*entity_id]);
			}
		}
		let mut action_id: usize = 1;
		for action in self.scene.actions.iter() {
			println!("{}. {}", action_id, self.render_action(& action));
			action_id = action_id + 1;
		}
	}

	fn render_detailed(&self) {
		println!("You see {}", self.components.descriptions[self.scene.location_id]);
		for entity_id in &self.scene.entity_ids {
			println!("{}", self.components.descriptions[*entity_id]);
		}
	}

	fn render_action(&self, action: &Action) -> String {
		return match action.action_type {
			ActionType::CHECK_INVENTORY => {
				return String::from("Check your inventory");
			}
			ActionType::GO => {
				return format!("Go to {}", self.components.names[action.arg_1.unwrap()]);
			}
			ActionType::LOOK => String::from("Look around"),
			ActionType::TAKE => {
				return format!("Take {}", self.components.names[action.arg_1.unwrap()]);
			}
			ActionType::TALK => {
				return format!("Speak to {}", self.components.names[action.arg_1.unwrap()]);
			}
		}
	}

	fn render_inventory(&self) {
		println!("In your inventory:");
		let entity_ids: Vec<usize> = self.components.locations[self.components.inventory_id].to_vec();
		for entity_id in entity_ids {
			println!("{}", self.components.names[entity_id]);
		}
	}

	fn update<'a>(&mut self, input: String) -> Result<(), &'a str> {
		if input == "quit" {
			return Err("Goodbye!");
		} else {
			let mut i: usize = input.parse::<usize>().unwrap();
			i = i - 1;
			let action = &self.scene.actions[i];
			self.state.last_action_type = action.action_type.clone();
			match action.action_type {
				ActionType::CHECK_INVENTORY => self.render_inventory(),
				ActionType::GO => {
					self.state.current_location = 
						self.components.destinations[action.arg_1.unwrap() - self.components.exits_start]
				},
				ActionType::LOOK => self.render_detailed(),
				ActionType::TAKE => {
					let id = action.arg_1.unwrap();
					let index = self.components.locations[self.state.current_location].iter().position(|eid| *eid == id).unwrap();
					self.components.locations[self.state.current_location].remove(index);
					self.components.locations[self.components.inventory_id].push(id);
					// record change
				},
				ActionType::TALK => ()
				// _ => ()
			}
		}
		return Ok(());
	}
}

fn get_input(prompt: &str) -> String{
	// println!("{}",prompt);
	let mut stdout = io::stdout();
	let _ = stdout.write_all(prompt.as_bytes());
	let _ = stdout.flush();
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_goes_into_input_above) => {},
		Err(_no_updates_is_fine) => {},
	}
	return input.trim().to_string();
}

#[macroquad::main("MyGame")]
async fn main() {

	// let mut game = Game {
	// 	components: data::make_components(),
	// 	scene: Scene{
	// 		location_id: 0,
	// 		entity_ids: Vec::new(),
	// 		exit_ids: Vec::new(),
	// 		takeable_item_ids: Vec::new(),
	// 		actions: Vec::new(),
	// 	},
	// 	state: State {
	// 		current_location: data::main::get_start_location_id(),
	// 		last_action_type: ActionType::GO,
	// 	},
	// };

	// data::main::load_data(&mut game.components);

	// loop {
	// 	game.setup_scene();
	// 	game.render();
	// 	let input: String = get_input("> ");
	// 	match game.update(input) {
	// 		Err(s) => { println!("{}", s); break; },
	// 		_ => ()
	// 	}
	// }

	// let mut app = Window::new("hello world", 1920, 1080);

	// app.set_color(255, 0, 255, 255);
	// app.draw_rect(Rect::new(
	// 	100,
	// 	110,
	// 	120,
	// 	130,
	// ));

	// while app.next_frame() {}

	loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}


//   = note:  "cc" "-m64" "/tmp/rustcWctBLs/symbols.o" "<6 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "/home/dmcblue/repos/simpleRPG/game/target/debug/deps/{libsimple-8e5f166cf68014c8.rlib,libsdl2-036102dbef4e200e.rlib,librand-351e4f2c4c4df3be.rlib,librand_xorshift-b234e5460f2c08eb.rlib,librand_pcg-f2f4d98c807b26e6.rlib,librand_hc-bb1be84f56d2a4af.rlib,librand_chacha-74e9e86618b85130.rlib,librand_isaac-d1a241b6c54747d2.rlib,librand_core-543e6033f4254532.rlib,librand_os-f9ac76fa909b0da8.rlib,librand_jitter-061216ef9e2942b9.rlib,librand_core-2863a1a60851be8f.rlib,libsdl2_sys-7e0e970abcd25588.rlib,libbitflags-e95052d8631a81a3.rlib,liblazy_static-0c9a5365c3da512b.rlib,libnum-47320607a47896af.rlib,libnum_rational-7f9afc9fb4984df9.rlib,libnum_bigint-5c164b72e2558b6f.rlib,libnum_complex-82fe0879865c179d.rlib,librustc_serialize-9152884cb6afe7bd.rlib,libnum_iter-c0affb480ebcca16.rlib,libnum_integer-baf778261a00f848.rlib,libnum_traits-42307217b1538fab.rlib,librand-c82415d2b7ec2746.rlib,librand-2eadf35e6c4f25b8.rlib,liblibc-5fa44a51d38a17f7.rlib}.rlib" "<sysroot>/lib64/rustlib/x86_64-unknown-linux-gnu/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*,libcfg_if-*,liblibc-*,liballoc-*,librustc_std_workspace_core-*,libcore-*,libcompiler_builtins-*}.rlib" "-Wl,-Bdynamic" "-lSDL2" "-lSDL2_image" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-L" "/tmp/rustcWctBLs/raw-dylibs" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "<sysroot>/lib64/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/dmcblue/repos/simpleRPG/game/target/debug/deps/simpleRPG-8713ae327ec91a1e" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
//   = note: some arguments are omitted. use `--verbose` to show all linker arguments
//   = note: /bin/ld: cannot find -lSDL2_image: No such file or directory
//           collect2: error: ld returned 1 exit status
