#[derive(PartialEq, Hash, Eq)]
pub enum QuestID {
    CombatVictory,
}

impl QuestID {
    fn name(&self) -> String {
        match self {
            QuestID::CombatVictory => "Win in Combat".to_string(),
        }
    }
}

pub struct Quest {
    pub quest_id: QuestID,
    finish_threshold: u32,
    // state
    value: u32,
    visible: bool, // can be tracked
    active: bool,  // progress can be made
    pub tracked: bool,
}

impl Quest {
    pub fn combat_victory() -> Self {
        Self {
            quest_id: QuestID::CombatVictory,
            finish_threshold: 1,
            value: 0,
            visible: true,
            active: true,
            tracked: true,
        }
    }

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
