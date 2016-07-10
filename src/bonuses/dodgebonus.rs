use bonuses;

/// An object which tracks dodge bonus values.
pub struct DodgeBonus {
	tracker: bonuses::StackingTracker,
}

impl DodgeBonus {
	/// Create an instance of DodgeBonus.
	pub fn new() -> DodgeBonus {
		DodgeBonus {
			tracker: bonuses::StackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for DodgeBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut DodgeBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut DodgeBonus {
		self.tracker.remove(amt);
		return self;
	}
}