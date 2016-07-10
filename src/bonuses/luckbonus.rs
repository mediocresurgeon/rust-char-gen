use bonuses;

/// An object which tracks luck bonus values.
pub struct LuckBonus {
	tracker: bonuses::NonStackingTracker,
}

impl LuckBonus {
	/// Create an instance of LuckBonus.
	pub fn new() -> LuckBonus {
		LuckBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for LuckBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut LuckBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut LuckBonus {
		self.tracker.remove(amt);
		return self;
	}
}