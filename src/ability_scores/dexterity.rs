use ability_scores;


pub struct Dexterity {
	score: ability_scores::GenericAbilityScore,
}

impl Default for Dexterity {
    fn default() -> Dexterity {
        Dexterity {
            score: ability_scores::GenericAbilityScore::default()
        }
    }
}

impl Dexterity {
    fn new(base_score: Option<u8>) -> Dexterity {
        Dexterity {
            score: ability_scores::GenericAbilityScore::new(base_score),
        }
    }
}

impl ability_scores::AbilityScore for Dexterity {
    fn total(&self) -> Option<u8> {
        return self.score.total();
    }

    fn bonus(&self) -> u8 {
        return self.score.bonus();
    }

    fn modifier(&self) -> i8 {
        return self.score.modifier();
    }

    fn get_base(&self) -> &Option<u8> {
        return self.score.get_base();
    }

    fn set_base(&mut self, base_score: Option<u8>) -> &mut Dexterity {
        self.score.set_base(base_score);
        return self;
    }

    fn add_penalty(&mut self, penalty: u8) -> &mut Dexterity {
        self.score.add_penalty(penalty);
        return self;
    }

    fn remove_penalty(&mut self, penalty: u8) -> &mut Dexterity {
        self.score.remove_penalty(penalty);
        return self;
    }

    fn get_penalty(&mut self) -> u8 {
        return self.score.get_penalty();
    }

    fn add_enhancement(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.add_enhancement(bonus);
        return self;
    }

    fn remove_enhancement(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.remove_enhancement(bonus);
        return self;
    }

    fn get_enhancement(&mut self) -> u8 {
        return self.score.get_enhancement();
    }

    fn add_inherent(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.add_inherent(bonus);
        return self;
    }

    fn remove_inherent(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.remove_inherent(bonus);
        return self;
    }

    fn get_inherent(&mut self) -> u8 {
        return self.score.get_inherent();
    }

    fn add_untyped(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.add_untyped(bonus);
        return self;
    }

    fn remove_untyped(&mut self, bonus: u8) -> &mut Dexterity {
        self.score.remove_untyped(bonus);
        return self;
    }

    fn get_untyped(&mut self) -> u8 {
        return self.score.get_untyped();
    }
}