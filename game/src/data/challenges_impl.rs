use std::collections::HashMap;
use super::challenges::{Challenge, ChallengeType, Phase, ChallengeCard, ChallengeEffect};
use super::components::{Components};

pub fn load_challenges(components: &mut Components) {
	components.challenge_types = HashMap::new();
	components.challenges = HashMap::new();
	components.challenge_cards = HashMap::new();

	components.challenges.insert(
		1763943190,
		Challenge{
			challenge_type_uuid: 1763946173,
			name: String::from("St. Yves Mount"),
			level: 1,
			phases: vec![
				Phase {
					name: String::from("Eastern Pass"),
					attributes: HashMap::from([(1763943193, 2),(1763943192, 4),(1763943191, 3),]),
					cards: vec![0],
				},
			],
		}
	);
	components.challenge_types.insert(
		1763943190,
		ChallengeType{
			name: String::from("Climbing"),
			attributes: HashMap::from([
				(String::from("skill"), 1763943193),
				(String::from("distance"), 1763943191),
				(String::from("toughness"), 1763943192),
			]),
		}
	);
}
