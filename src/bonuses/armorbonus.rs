use bonuses;

/// An object which tracks armor bonus values.
pub struct ArmorBonus {
	tracker: bonuses::NonStackingTracker,
}

impl ArmorBonus {
	/// Create an instance of ArmorBonus.
	pub fn new() -> ArmorBonus {
		ArmorBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for ArmorBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut ArmorBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut ArmorBonus {
		self.tracker.remove(amt);
		return self;
	}
}