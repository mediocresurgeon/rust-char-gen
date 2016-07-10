use bonuses;

/// An object which tracks deflection bonus values.
pub struct DeflectionBonus {
	tracker: bonuses::NonStackingTracker,
}

impl DeflectionBonus {
	/// Create an instance of DeflectionBonus.
	pub fn new() -> DeflectionBonus {
		DeflectionBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for DeflectionBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut DeflectionBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut DeflectionBonus {
		self.tracker.remove(amt);
		return self;
	}
}