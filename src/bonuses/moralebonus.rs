use bonuses;

/// An object which tracks morale bonus values.
pub struct MoraleBonus {
	tracker: bonuses::NonStackingTracker,
}

impl MoraleBonus {
	/// Create an instance of MoraleBonus.
	pub fn new() -> MoraleBonus {
		MoraleBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for MoraleBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut MoraleBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut MoraleBonus {
		self.tracker.remove(amt);
		return self;
	}
}