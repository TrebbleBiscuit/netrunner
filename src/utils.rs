// use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn roll_encounter(success_chance: f32) -> bool {
    // true if successful - chance is between 0 and 1
    let roll: f32 = thread_rng().gen();
    return roll > success_chance;
}
