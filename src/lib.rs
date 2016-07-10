mod bonuses;

pub mod chargen {
	use bonuses;

	pub fn do_something() {
		let mut bt = bonuses::EnhancementBonus::new();
		println!("Hello, world!");
	}
}