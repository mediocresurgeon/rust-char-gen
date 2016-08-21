use bonuses;

/// An object which tracks insight bonus values.
pub struct InsightBonus {
	tracker: bonuses::NonStackingTracker,
}

impl InsightBonus {
	/// Create an instance of InsightBonus.
	pub fn new() -> InsightBonus {
		InsightBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for InsightBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut InsightBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut InsightBonus {
		self.tracker.remove(amt);
		return self;
	}
}