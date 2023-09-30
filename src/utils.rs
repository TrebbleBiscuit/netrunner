// use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn roll_encounter(chance: f32) -> bool {
    let roll: f32 = thread_rng().gen();
    return roll > chance;
}
