use bonuses;

/// An object which tracks sacred bonus values.
pub struct SacredBonus {
	tracker: bonuses::NonStackingTracker,
}

impl SacredBonus {
	/// Create an instance of SacredBonus.
	pub fn new() -> SacredBonus {
		SacredBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for SacredBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut SacredBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut SacredBonus {
		self.tracker.remove(amt);
		return self;
	}
}