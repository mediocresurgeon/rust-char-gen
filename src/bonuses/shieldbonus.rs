use bonuses;

/// An object which tracks shield bonus values.
pub struct ShieldBonus {
	tracker: bonuses::NonStackingTracker,
}

impl ShieldBonus {
	/// Create an instance of ShieldBonus.
	pub fn new() -> ShieldBonus {
		ShieldBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for ShieldBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut ShieldBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut ShieldBonus {
		self.tracker.remove(amt);
		return self;
	}
}