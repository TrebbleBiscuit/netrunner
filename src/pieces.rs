use rand::{random, seq::SliceRandom, thread_rng};
use std::fmt;

pub const BASE_SKILL_POINTS: i32 = 10;

fn random_hostile_name() -> String {
    let vs: Vec<&str> = vec![
        "adware-imp",
        "maniabot",
        "SpamSpyder",
        "darknet-dragon",
        "silent-strike",
        "phantom_protocol",
    ];
    return vs.choose(&mut thread_rng()).unwrap().to_string();
}

#[derive(Debug)]
pub enum Disposition {
    Neutral,
    Hostile,
}

impl fmt::Display for Disposition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Contact {
    pub name: String,
    pub hp: CappedValue,
    pub skills: Skills,
    pub disposition: Disposition,
}

impl Contact {
    pub fn new_from_level(level: i32) -> Self {
        let total_skill_points = BASE_SKILL_POINTS * level;
        let range: f32 = total_skill_points as f32 / 4.0;
        let r_skill: i32 = ((total_skill_points as f32 / 2.0)
            + ((random::<f32>() - 0.5) * 2.0 * range))
            .round() as i32;
        let health = 25 + (level * 5);
        Self {
            name: random_hostile_name(),
            hp: CappedValue::new_health(health),
            skills: Skills {
                hacking: r_skill,
                firewall: total_skill_points - r_skill,
            },
            disposition: Disposition::Hostile,
        }
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            name: random_hostile_name(),
            hp: CappedValue::new_health(30),
            skills: Skills::default(),
            disposition: Disposition::Hostile,
        }
    }
}

pub enum CappedValueType {
    Health,
    Ram,
}

pub struct CappedValue {
    pub value: i32,
    pub upper_limit: i32,
    pub value_type: CappedValueType,
}

impl CappedValue {
    pub fn new_health(value: i32) -> Self {
        Self {
            value: value,
            upper_limit: value,
            value_type: CappedValueType::Health,
        }
    }

    pub fn new_ram(value: i32, upper_limit: i32) -> Self {
        Self {
            value: value,
            upper_limit: upper_limit,
            value_type: CappedValueType::Ram,
        }
    }

    fn hit_zero(&self) {
        // match self.value_type {
        //     CappedValueType::Health => {
        //         println!("oh no you're dead")
        //     }
        //     CappedValueType::Ram => println!("oh no you're out of RAM"),
        // }
    }

    pub fn change_by(&mut self, amount: i32) {
        self.value = self.upper_limit.min((self.value + amount).max(0));
        if self.value == 0 {
            self.hit_zero();
        }
    }
}

pub struct Skills {
    pub hacking: i32,
    pub firewall: i32,
}

impl Skills {
    pub fn total_points(&self) -> i32 {
        return self.hacking + self.firewall;
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            hacking: 4,
            firewall: 4,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Networks {
    Internet,
    SIPRnet,
}

impl Networks {
    pub fn difficulty(&self) -> f32 {
        match *self {
            Networks::Internet => 1.0,
            Networks::SIPRnet => 3.0,
        }
    }
}

impl fmt::Display for Networks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
