use bonuses;

/// An object which tracks resistance bonus values.
pub struct ResistanceBonus {
	tracker: bonuses::NonStackingTracker,
}

impl ResistanceBonus {
	/// Create an instance of ResistanceBonus.
	pub fn new() -> ResistanceBonus {
		ResistanceBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for ResistanceBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut ResistanceBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut ResistanceBonus {
		self.tracker.remove(amt);
		return self;
	}
}