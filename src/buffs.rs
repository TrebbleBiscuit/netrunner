#[derive(PartialEq)]
pub enum BuffType {
    MalwareInjected,
    FirewallFortified,
    Overclock,
}

pub struct Buff {
    buff_type: BuffType,
    duration: u32,
}

impl Buff {
    fn new(buff_type: BuffType, duration: u32) -> Self {
        Buff {
            buff_type,
            duration,
        }
    }

    fn add_duration(&mut self, additional_duration: u32) {
        self.duration += additional_duration;
    }

    fn decrease_duration(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
        }
    }

    fn has_expired(&self) -> bool {
        self.duration == 0
    }
}

pub struct BuffContainer {
    buffs: Vec<Buff>,
}

impl BuffContainer {
    pub fn new() -> Self {
        BuffContainer { buffs: Vec::new() }
    }

    pub fn add_buff(&mut self, buff_type: BuffType, duration: u32) {
        if let Some(buff) = self.buffs.iter_mut().find(|b| b.buff_type == buff_type) {
            buff.add_duration(duration);
        } else {
            self.buffs.push(Buff::new(buff_type, duration));
        }
    }

    pub fn has_buff(&self, buff_type: BuffType) -> Option<u32> {
        if let Some(buff) = self.buffs.iter().find(|b| b.buff_type == buff_type) {
            Some(buff.duration)
        } else {
            None
        }
    }

    pub fn do_turn(&mut self) {
        // Iterate over the buffs vector and decrease duration
        for buff in &mut self.buffs {
            buff.decrease_duration();
        }

        // Remove expired buffs
        self.buffs.retain(|buff| !buff.has_expired());
    }

    pub fn get_buff_dmg(&self, base_dmg: i32) -> i32 {
        let mut multiplier = 0.0;
        for buff in self.buffs.iter() {
            match buff.buff_type {
                BuffType::MalwareInjected => multiplier -= 0.2,
                BuffType::FirewallFortified => {}
                BuffType::Overclock => multiplier += 1.1,
            }
        }
        return (base_dmg as f32 * multiplier).ceil() as i32;
    }

    pub fn get_buff_ram(&self, base_ram: i32) -> i32 {
        let mut multiplier = 1.0;
        for buff in self.buffs.iter() {
            match buff.buff_type {
                BuffType::MalwareInjected => multiplier += 0.2,
                BuffType::FirewallFortified => {}
                BuffType::Overclock => multiplier += 0.9,
            }
        }
        return (base_ram as f32 * multiplier).ceil() as i32;
    }

    pub fn clear(&mut self) {
        self.buffs.clear();
    }
}
