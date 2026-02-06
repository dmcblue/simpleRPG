// Used by the builder to track values

pub struct Count {
	pub uuids: Vec<usize>,
	pub start: usize,
	pub end: usize,
}

impl Count {
	pub fn new() -> Self {
		return Self {
			uuids: Vec::new(),
			start: 0,
			end: 0,
		};
	}

	pub fn in_range(&self, index: usize) -> bool {
		return index >= self.start && index < self.end;
	}
}

pub struct Counts {
	pub challenge_cards: Count,
	pub challenge_types: Count,
	pub challenges: Count,
	pub conversations: Count,
	pub exits: Count,
	pub inventory_uuid: usize,
	pub items: Count,
	pub locations: Count,
	pub people: Count,
	pub player_cards: Count,
	pub vending_ether_uuid: usize,
	pub vending: Count,
	pub starting_location_uuid: usize,
	pub total: usize,
}

impl Counts {
	pub fn new() -> Self {
		Self {
			challenge_cards: Count::new(),
			challenge_types: Count::new(),
			challenges: Count::new(),
			conversations: Count::new(),
			exits: Count::new(),
			inventory_uuid: 0,
			items: Count::new(),
			locations: Count::new(),
			people: Count::new(),
			player_cards: Count::new(),
			vending_ether_uuid: 0,
			vending: Count::new(),
			starting_location_uuid: 0,
			total: 0,
		}
	}
}
