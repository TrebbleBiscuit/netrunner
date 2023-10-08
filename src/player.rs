use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

use crate::pieces::{CappedValue, Networks, Skills, BASE_SKILL_POINTS};

pub struct PlayerStats {
    pub kills: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { kills: 0 }
    }
}

pub struct NetStats {
    pub total_intel: f32,
}

impl Default for NetStats {
    fn default() -> Self {
        Self { total_intel: 0.0 }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerUpgradeType {
    HPMaxUp,
}

impl PlayerUpgradeType {
    pub fn name(&self) -> String {
        match *self {
            PlayerUpgradeType::HPMaxUp => "HP Max +".to_string(),
        }
    }
}

pub struct PlayerUpgrade {
    upgrade_type: PlayerUpgradeType,
    pub level: u32,
    base_cost: u32,
    cost_per_level: u32,
}

impl PlayerUpgrade {
    pub fn cost(&self) -> u32 {
        return self.base_cost + (self.level * self.cost_per_level);
    }

    pub fn upgrade_type(&self) -> PlayerUpgradeType {
        match self.upgrade_type {
            PlayerUpgradeType::HPMaxUp => PlayerUpgradeType::HPMaxUp,
        }
    }
}

pub struct Player {
    pub name: String,
    pub stats: PlayerStats, // track for posterity
    pub net_stats: HashMap<Networks, NetStats>,
    pub skills: Skills, // skills for checks and such
    pub hp: CappedValue,
    pub ram: CappedValue,
    pub credits: i32,
    pub xp: i32,
    pub upgrades: HashMap<PlayerUpgradeType, PlayerUpgrade>,
}

impl Player {
    pub fn available_skill_points(&self) -> i32 {
        // debug - for now, 12 points is the max
        return BASE_SKILL_POINTS - self.skills.total_points();
    }
}

impl Default for Player {
    fn default() -> Self {
        let mut upgrades = HashMap::new();
        upgrades.insert(
            PlayerUpgradeType::HPMaxUp,
            PlayerUpgrade {
                upgrade_type: PlayerUpgradeType::HPMaxUp,
                level: 0,
                cost_per_level: 50,
                base_cost: 100,
            },
        );
        let mut net_stats = HashMap::new();
        net_stats.insert(Networks::Internet, NetStats::default());
        net_stats.insert(Networks::SIPRnet, NetStats::default());
        Self {
            name: random_default_name(),
            stats: PlayerStats::default(),
            net_stats: net_stats,
            skills: Skills::default(),
            hp: CappedValue::new_health(100),
            ram: CappedValue::new_ram(50, 100),
            credits: 100,
            xp: 0,
            upgrades: upgrades,
        }
    }
}

fn random_default_name() -> String {
    let vs: Vec<&str> = vec![
        "riftrunner",
        "astralByte",
        "digital-nomad",
        "pulse-echo",
        "ShadowSync",
        "NovaHaxD",
        "CYPHER",
        "Aki Zeta-5",
        "Prime Function",
        "Nexus-11",
    ];
    return vs.choose(&mut thread_rng()).unwrap().to_string();
}
