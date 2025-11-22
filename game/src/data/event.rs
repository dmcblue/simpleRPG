#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EventType {
	ENABLE_CONVERSATION,
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
	pub event_type: EventType,
	pub arg_1: Option<usize>,
}

impl Default for Event {
	fn default() -> Event {
		Event {
			event_type: EventType::ENABLE_CONVERSATION,
			arg_1: None,
			// arg_2: None,
			// arg_3: None,
		}
	}
}