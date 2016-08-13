mod bonuses;
mod characters;

pub mod chargen {
	pub fn do_something() {
		use bonuses;
		let mut bt = bonuses::EnhancementBonus::new();
		println!("Hello, world!");
	}
}