use bonuses;

/// An object which tracks enhancement bonus values.
pub struct EnhancementBonus {
	tracker: bonuses::NonStackingTracker,
}

impl EnhancementBonus {
	/// Create an instance of EnhancementBonus.
	pub fn new() -> EnhancementBonus {
		EnhancementBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for EnhancementBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut EnhancementBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut EnhancementBonus {
		self.tracker.remove(amt);
		return self;
	}
}