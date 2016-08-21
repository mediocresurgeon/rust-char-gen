use bonuses;
use bonuses::BonusTracker;
pub use self::charisma::{ Charisma };
pub use self::constitution::{ Constitution };
pub use self::dexterity::{ Dexterity };
pub use self::intelligence::{ Intelligence };
pub use self::strength::{ Strength };
pub use self::wisdom::{ Wisdom };
mod charisma;
mod constitution;
mod dexterity;
mod intelligence;
mod strength;
mod wisdom;


pub trait AbilityScore {
    fn total(&self) -> Option<u8> where Self: Sized;
    fn bonus(&self) -> u8 where Self: Sized;
    fn modifier(&self) -> i8 where Self: Sized;
    
    fn get_base(&self) -> &Option<u8> where Self: Sized;
    fn set_base(&mut self, Option<u8>) -> &mut Self where Self: Sized;
    
    fn add_penalty(&mut self, u8) -> &mut Self where Self: Sized;
    fn remove_penalty(&mut self, u8) -> &mut Self where Self: Sized;
    fn get_penalty(&mut self) -> u8 where Self: Sized;

    fn add_enhancement(&mut self, u8) -> &mut Self where Self: Sized;
    fn remove_enhancement(&mut self, u8) -> &mut Self where Self: Sized;
    fn get_enhancement(&mut self) -> u8 where Self: Sized;

    fn add_inherent(&mut self, u8) -> &mut Self where Self: Sized;
    fn remove_inherent(&mut self, u8) -> &mut Self where Self: Sized;
    fn get_inherent(&mut self) -> u8 where Self: Sized;

    fn add_untyped(&mut self, u8) -> &mut Self where Self: Sized;
    fn remove_untyped(&mut self, u8) -> &mut Self where Self: Sized;
    fn get_untyped(&mut self) -> u8 where Self: Sized;
}

struct GenericAbilityScore {
    base: Option<u8>,
    penalties: bonuses::Penalty,
    enhancement_bonus: bonuses::EnhancementBonus,
    inherent_bonus: bonuses::InherentBonus,
    untyped_bonus: bonuses::UntypedBonus,
}

impl Default for GenericAbilityScore {
    fn default() -> GenericAbilityScore {
        GenericAbilityScore {
            base: Some(10),
            penalties: bonuses::Penalty::new(),
            enhancement_bonus: bonuses::EnhancementBonus::new(),
            inherent_bonus: bonuses::InherentBonus::new(),
            untyped_bonus: bonuses::UntypedBonus::new(),
        }
    }
}

impl GenericAbilityScore {
    fn new(base_score: Option<u8>) -> GenericAbilityScore {
        GenericAbilityScore {
            base: base_score,
            penalties: bonuses::Penalty::new(),
            enhancement_bonus: bonuses::EnhancementBonus::new(),
            inherent_bonus: bonuses::InherentBonus::new(),
            untyped_bonus: bonuses::UntypedBonus::new(),
        }
    }

    fn get_modifier(score: Option<u8>) -> i8 {
        match score {
            Some(value) => {
                let cast_value: f32 = value as f32;
                let total = cast_value / 2.0 - 5.0;
                return total.floor() as i8;
            },
            None => {
                return 0;
            },
        }
    }
}

impl AbilityScore for GenericAbilityScore {
    fn total(&self) -> Option<u8> {
        match self.base {
            None => { return None }, 
            Some(base) => {
                let mut running_total = base as i8;
                running_total -= self.penalties.total() as i8;
                running_total += self.enhancement_bonus.total() as i8;
                running_total += self.inherent_bonus.total() as i8;
                running_total += self.untyped_bonus.total() as i8;
                if 0 < running_total {
                    return Some(running_total as u8);
                }
                return Some(0);
            },
        }
    }

    fn bonus(&self) -> u8 {
        let modifier = GenericAbilityScore::get_modifier(self.base);
        if 0 > modifier {
            return 0;
        } else {
            return modifier as u8;
        }
    }

    fn modifier(&self) -> i8 {
        GenericAbilityScore::get_modifier(self.base)
    }

    fn get_base(&self) -> &Option<u8> {
        &self.base
    }

    fn set_base(&mut self, base: Option<u8>) -> &mut GenericAbilityScore {
        self.base = base;
        return self;
    }

    fn add_penalty(&mut self, penalty: u8) -> &mut GenericAbilityScore {
        self.penalties.add(penalty);
        return self;
    }

    fn remove_penalty(&mut self, penalty: u8) -> &mut GenericAbilityScore {
        self.penalties.remove(penalty);
        return self;
    }
    
    fn get_penalty(&mut self) -> u8 {
        return self.penalties.total();
    }

    fn add_enhancement(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.enhancement_bonus.add(bonus);
        return self;
    }

    fn remove_enhancement(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.enhancement_bonus.remove(bonus);
        return self;
    }
    
    fn get_enhancement(&mut self) -> u8 {
        return self.enhancement_bonus.total();
    }

    fn add_inherent(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.inherent_bonus.add(bonus);
        return self;
    }

    fn remove_inherent(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.inherent_bonus.remove(bonus);
        return self;
    }
    
    fn get_inherent(&mut self) -> u8 {
        return self.inherent_bonus.total();
    }

    fn add_untyped(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.untyped_bonus.add(bonus);
        return self;
    }

    fn remove_untyped(&mut self, bonus: u8) -> &mut GenericAbilityScore {
        self.untyped_bonus.remove(bonus);
        return self;
    }
    
    fn get_untyped(&mut self) -> u8 {
        return self.untyped_bonus.total();
    }
}

#[cfg(test)]
mod genericabilityscoretests {
    use super::*;
    use ability_scores;

    #[test]
    fn default_get_base() {
        // the default abilityscore should be 10
        let score = ability_scores::GenericAbilityScore::default();
        let expected_value: Option<u8> = Some(10);
        assert_eq!(&expected_value, score.get_base());
    }

    #[test]
    fn new_get_base() {
        // get_base should return the score set in the constructor
        let default_base: Option<u8> = Some(18);
        let score = ability_scores::GenericAbilityScore::new(default_base);
        assert_eq!(&default_base, score.get_base());
    }

    #[test]
    fn set_base_get_base() {
        // get_base should return the score set in set_base
        let mut score = ability_scores::GenericAbilityScore::default();
        let new_base: Option<u8> = Some(18);
        score.set_base(new_base);
        assert_eq!(&new_base, score.get_base());
    }

    #[test]
    fn none_bonus() {
        let score = ability_scores::GenericAbilityScore::new(None);
        let expected_bonus: u8 = 0;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn zero_bonus() {
        // null ability scores have a bonus of 0
        let score = ability_scores::GenericAbilityScore::new(Some(0));
        let expected_bonus: u8 = 0;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn nine_bonus() {
        // Ability scores less than 10 have a bonus of 0
        let score = ability_scores::GenericAbilityScore::new(Some(9));
        let expected_bonus: u8 = 0;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn ten_bonus() {
        // Ability score of 10 has a bonus of 0
        let score = ability_scores::GenericAbilityScore::new(Some(10));
        let expected_bonus: u8 = 0;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn eleven_bonus() {
        // Ability score of 11 has a bonus of 0
        let score = ability_scores::GenericAbilityScore::new(Some(11));
        let expected_bonus: u8 = 0;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn twelve_bonus() {
        // Ability score of 12 has a bonus of 1
        let score = ability_scores::GenericAbilityScore::new(Some(12));
        let expected_bonus: u8 = 1;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn eighteen_bonus() {
        // Ability score of 18 has a bonus of 1
        let score = ability_scores::GenericAbilityScore::new(Some(18));
        let expected_bonus: u8 = 4;
        assert_eq!(expected_bonus, score.bonus());
    }

    #[test]
    fn zero_modifier() {
        // null ability scores have a modifier of 0
        let score = ability_scores::GenericAbilityScore::new(Some(0));
        let expected_bonus: i8 = -5;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn nine_modifier() {
        // Ability scores of 9 has a modifier of -1
        let score = ability_scores::GenericAbilityScore::new(Some(9));
        let expected_bonus: i8 = -1;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn ten_modifier() {
        // Ability score of 10 has a modifier of 0
        let score = ability_scores::GenericAbilityScore::new(Some(10));
        let expected_bonus: i8 = 0;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn eleven_modifier() {
        // Ability score of 11 has a modifier of 0
        let score = ability_scores::GenericAbilityScore::new(Some(11));
        let expected_bonus: i8 = 0;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn twelve_modifier() {
        // Ability score of 12 has a modifier of 1
        let score = ability_scores::GenericAbilityScore::new(Some(12));
        let expected_bonus: i8 = 1;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn eighteen_modifier() {
        // Ability score of 18 has a modifier of 1
        let score = ability_scores::GenericAbilityScore::new(Some(18));
        let expected_bonus: i8 = 4;
        assert_eq!(expected_bonus, score.modifier());
    }

    #[test]
    fn ten_base_total() {
        // total should return the score set in the constructor
        let base: Option<u8> = Some(10);
        let score = ability_scores::GenericAbilityScore::new(base);
        assert_eq!(base, score.total());
    }

    #[test]
    fn eighteen_base_total() {
        // total should return the score set in the constructor
        let base: Option<u8> = Some(18);
        let score = ability_scores::GenericAbilityScore::new(base);
        assert_eq!(base, score.total());
    }

    #[test]
    fn penalty_add_total() {
        // total should return the sume of the base score and the enhancement bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_penalty(2);
        let expected_total: Option<u8> = Some(8);
        assert_eq!(expected_total, score.total());
    }

    #[test]
    fn penalty_add_negative_total() {
        // total should return the sume of the base score and the enhancement bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_penalty(11);
        let expected_total: Option<u8> = Some(0);
        assert_eq!(expected_total, score.total());
    }

    #[test]
    fn penalty_remove_total() {
        // total should return the sume of the base score and the enhancement bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_penalty(2);
        score.remove_penalty(2);
        assert_eq!(base, score.total());
    }

    #[test]
    fn enhancement_add_total() {
        // total should return the sume of the base score and the enhancement bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_enhancement(2);
        let expected_total: Option<u8> = Some(12);
        assert_eq!(expected_total, score.total());
    }

    #[test]
    fn enhancement_remove_total() {
        // total should return the sume of the base score and the enhancement bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_enhancement(2);
        score.remove_enhancement(2);
        assert_eq!(base, score.total());
    }

    #[test]
    fn inherent_add_total() {
        // total should return the sume of the base score and the inherent bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_inherent(2);
        let expected_total: Option<u8> = Some(12);
        assert_eq!(expected_total, score.total());
    }

    #[test]
    fn inherent_remove_total() {
        // total should return the sume of the base score and the inherent bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_inherent(2);
        score.remove_inherent(2);
        assert_eq!(base, score.total());
    }

    #[test]
    fn untyped_add_total() {
        // total should return the sume of the base score and the untyped bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_untyped(2);
        let expected_total: Option<u8> = Some(12);
        assert_eq!(expected_total, score.total());
    }

    #[test]
    fn untyped_remove_total() {
        // total should return the sume of the base score and the untyped bonus
        let base: Option<u8> = Some(10);
        let mut score = ability_scores::GenericAbilityScore::new(base);
        score.add_untyped(2);
        score.remove_untyped(2);
        assert_eq!(base, score.total());
    }
}