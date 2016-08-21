pub mod bonuses;
pub mod characters;
pub mod ability_scores;


pub mod character_creator {
	use bonuses;
	use characters;

	/// Builds and returns a character.
	pub fn return_something() -> Box<characters::Character> {
		return Box::new(characters::Humanoid::new());
	}
}