use bonuses;

/// An object which tracks natural armor enhancement bonus values.
pub struct NaturalArmorEnhancementBonus {
	tracker: bonuses::NonStackingTracker,
}

impl NaturalArmorEnhancementBonus {
	/// Create an instance of NaturalArmorEnhancementBonus.
	pub fn new() -> NaturalArmorEnhancementBonus {
		NaturalArmorEnhancementBonus {
			tracker: bonuses::NonStackingTracker::new()
		}
	}
}

impl bonuses::BonusTracker for NaturalArmorEnhancementBonus {
	/// Returns the total bonus.
	fn total(&self) -> u8 {
		return self.tracker.total();
	}

	/// Adds a value.
	fn add(&mut self, amt: u8) -> &mut NaturalArmorEnhancementBonus {
		self.tracker.add(amt);
		return self;
	}

	/// Removes a value.
	fn remove(&mut self, amt: u8) -> &mut NaturalArmorEnhancementBonus {
		self.tracker.remove(amt);
		return self;
	}
}