use bonuses;

/// An object which tracks circumstance bonus values.
pub struct CircumstanceBonus {
	tracker: bonuses::StackingTracker,
}

impl CircumstanceBonus {
	/// Create an instance of CircumstanceBonus.
	pub fn new() -> CircumstanceBonus {
		CircumstanceBonus {
			tracker: bonuses::StackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for CircumstanceBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut CircumstanceBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut CircumstanceBonus {
		self.tracker.remove(amt);
		return self;
	}
}