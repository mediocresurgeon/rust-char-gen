use bonuses;

/// An object which tracks profane bonus values.
pub struct ProfaneBonus {
	tracker: bonuses::NonStackingTracker,
}

impl ProfaneBonus {
	/// Create an instance of ProfaneBonus.
	pub fn new() -> ProfaneBonus {
		ProfaneBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for ProfaneBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut ProfaneBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut ProfaneBonus {
		self.tracker.remove(amt);
		return self;
	}
}