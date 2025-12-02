pub struct ChallengeType {
	pub name: String,
	// name -> uuid
	pub attributes: HashMep<String, usize>,
}

pub struct Challenge {
	pub challenge_type: &ChallengeType,
	pub name: String,
	pub level: usize,
	pub phases: Vec<Phase>,
}

pub struct Phase {
	pub name: String,
	// uuid -> value
	pub attributes: HashMap<usize, usize>,
	// uuid
	pub cards: Vec<usize>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChallengeEffectType {
	TEMP_BUFF,
}

pub struct ChallengeCard {
	// pub id: usize,
	pub challenge_type: &ChallengeType,
	pub name: String,
	pub effects: Vec<ChallengeEffect>,
}

#[derive(Clone, Copy, Debug)]
pub struct ChallengeEffect {
	pub event_type: ChallengeEventType,
	pub arg_1: usize,
	pub arg_2: usize,
	pub arg_3: usize,
}