use bonuses;

/// An object which tracks alchemical bonus values.
pub struct AlchemicalBonus {
	tracker: bonuses::NonStackingTracker,
}

impl AlchemicalBonus {
	/// Create an instance of AlchemicalBonus.
	pub fn new() -> AlchemicalBonus {
		AlchemicalBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for AlchemicalBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut AlchemicalBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut AlchemicalBonus {
		self.tracker.remove(amt);
		return self;
	}
}