use bonuses;

/// An object which tracks racial bonus values.
pub struct RacialBonus {
	tracker: bonuses::StackingTracker,
}

impl RacialBonus {
	/// Create an instance of RacialBonus.
	pub fn new() -> RacialBonus {
		RacialBonus {
			tracker: bonuses::StackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for RacialBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut RacialBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut RacialBonus {
		self.tracker.remove(amt);
		return self;
	}
}