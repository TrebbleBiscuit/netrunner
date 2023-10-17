use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

use crate::buffs::BuffContainer;
use crate::pieces::{CappedValue, Networks, Skills, BASE_SKILL_POINTS};
use crate::quests::{default_quests, Quest, QuestID};

pub struct PlayerStats {
    pub kills: u32,
    pub datamine_success: u32,
    pub search_success: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            kills: 0,
            datamine_success: 0,
            search_success: 0,
        }
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PlayerUpgradeType {
    HPMaxUp,
    SecurityUp,
}

impl PlayerUpgradeType {
    pub fn name(&self) -> String {
        match *self {
            PlayerUpgradeType::HPMaxUp => "HP Max +".to_string(),
            PlayerUpgradeType::SecurityUp => "Sec Max +".to_string(),
        }
    }
}

pub struct PlayerUpgrade {
    pub upgrade_type: PlayerUpgradeType,
    pub level: u32,
    base_cost: u32,
    cost_per_level: u32,
    pub available: bool,
}

impl PlayerUpgrade {
    pub fn cost(&self) -> u32 {
        return self.base_cost + (self.level * self.cost_per_level);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerFlag {
    DiscoveredShopBasic,
    EditingTrackedQuests,
    UnlockedNetworkSIPR,
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
    pub flags: Vec<PlayerFlag>,
    pub buffs: BuffContainer,
    pub quests: HashMap<QuestID, Quest>,
}

impl Player {
    pub fn available_skill_points(&self) -> i32 {
        // debug - for now, 12 points is the max
        return BASE_SKILL_POINTS - self.skills.total_points();
    }

    pub fn enable_flag(&mut self, flag: PlayerFlag) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
    }

    pub fn disable_flag(&mut self, flag: &PlayerFlag) {
        self.flags.retain(|f| f != flag);
    }

    pub fn has_flag(&self, flag: &PlayerFlag) -> bool {
        self.flags.contains(flag)
    }

    pub fn toggle_flag(&mut self, flag: PlayerFlag) {
        if self.flags.contains(&flag) {
            self.disable_flag(&flag);
        } else {
            self.flags.push(flag);
        }
    }

    pub fn add_xp(&mut self, to_add: i32) {
        self.xp += to_add;
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
                available: true,
            },
        );
        upgrades.insert(
            PlayerUpgradeType::SecurityUp,
            PlayerUpgrade {
                upgrade_type: PlayerUpgradeType::SecurityUp,
                level: 0,
                cost_per_level: 300,
                base_cost: 150,
                available: false,
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
            credits: 0,
            xp: 0,
            upgrades: upgrades,
            flags: vec![],
            buffs: BuffContainer::new(),
            quests: default_quests(),
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
