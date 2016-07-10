use bonuses;

/// An object which tracks competence bonus values.
pub struct CompetenceBonus {
	tracker: bonuses::NonStackingTracker,
}

impl CompetenceBonus {
	/// Create an instance of CompetenceBonus.
	pub fn new() -> CompetenceBonus {
		CompetenceBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for CompetenceBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut CompetenceBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut CompetenceBonus {
		self.tracker.remove(amt);
		return self;
	}
}