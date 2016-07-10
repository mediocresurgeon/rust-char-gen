use bonuses;

/// An object which tracks inherent bonus values.
pub struct InherentBonus {
	tracker: bonuses::NonStackingTracker,
}

impl InherentBonus {
	/// Create an instance of InherentBonus.
	pub fn new() -> InherentBonus {
		InherentBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for InherentBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut InherentBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut InherentBonus {
		self.tracker.remove(amt);
		return self;
	}
}