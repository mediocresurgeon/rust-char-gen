use bonuses;

/// An object which tracks size bonus values.
pub struct SizeBonus {
	tracker: bonuses::NonStackingTracker,
}

impl SizeBonus {
	/// Create an instance of SizeBonus.
	pub fn new() -> SizeBonus {
		SizeBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for SizeBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut SizeBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut SizeBonus {
		self.tracker.remove(amt);
		return self;
	}
}