use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq)]
pub enum QuestID {
    CombatVictory,
    DatamineSuccess,
}

impl QuestID {
    fn name(&self) -> String {
        match self {
            QuestID::CombatVictory => "Win in Combat".to_string(),
            QuestID::DatamineSuccess => "Successfully datamine".to_string(),
        }
    }
}

pub enum QuestReward {
    XP(i32),
}

pub struct Quest {
    pub quest_id: QuestID,
    finish_threshold: u32,
    pub reward: QuestReward,
    // state
    value: u32,
    visible: bool, // can be tracked
    active: bool,  // progress can be made
    pub tracked: bool,
}

pub fn default_quests() -> HashMap<QuestID, Quest> {
    let mut quests = HashMap::new();
    quests.insert(
        QuestID::CombatVictory,
        Quest {
            quest_id: QuestID::CombatVictory,
            finish_threshold: 4,
            reward: QuestReward::XP(80),
            value: 0,
            visible: true,
            active: true,
            tracked: true,
        },
    );
    quests.insert(
        QuestID::DatamineSuccess,
        Quest {
            quest_id: QuestID::DatamineSuccess,
            finish_threshold: 10,
            reward: QuestReward::XP(100),
            value: 0,
            visible: true,
            active: true,
            tracked: true,
        },
    );
    return quests;
}

impl Quest {
    pub fn name(&self) -> String {
        format!(
            "{}x {}",
            self.finish_threshold - self.value,
            self.quest_id.name()
        )
    }

    /// increment value if active
    pub fn increment(&mut self) {
        if self.active {
            self.value += 1;
        };
    }

    pub fn is_finished(&self) -> bool {
        self.value >= self.finish_threshold
    }

    pub fn trackable(&self) -> bool {
        return self.visible && !self.is_finished();
    }
}
