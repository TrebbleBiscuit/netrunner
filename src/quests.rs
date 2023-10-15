#[derive(PartialEq)]
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
    quest_id: QuestID,
    finish_threshold: u32,
    // state
    value: u32,
    visible: bool, // can be tracked
    active: bool,  // progress can be made
}

impl Quest {
    /// increment value if active
    fn increment(&mut self) {
        if self.active {
            self.value += 1;
        };
    }

    fn finished(&self) -> bool {
        self.value >= self.finish_threshold
    }
}

pub struct Quests {
    quests: Vec<Quest>,
}

impl Quests {
    pub fn new() -> Self {
        Quests { quests: Vec::new() }
    }
}
