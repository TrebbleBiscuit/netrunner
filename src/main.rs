#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rand::seq::SliceRandom;
use rand::{random, thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

mod utils;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(450.0, 350.0)),
        ..Default::default()
    };
    eframe::run_native(
        "cybergame",
        options,
        Box::new(|_cc| Box::<NetrunnerGame>::default()),
    )
}

#[derive(Debug)]
enum Disposition {
    Neutral,
    Hostile,
}

impl fmt::Display for Disposition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Contact {
    name: String,
    hp: CappedValue,
    skills: Skills,
    disposition: Disposition,
}

impl Contact {
    fn new_from_level(level: i32) -> Self {
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

struct PlayerStats {
    kills: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { kills: 0 }
    }
}

struct NetStats {
    total_intel: f32,
}

impl Default for NetStats {
    fn default() -> Self {
        Self { total_intel: 0.0 }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum PlayerUpgradeType {
    HPMaxUp,
}

impl PlayerUpgradeType {
    fn name(&self) -> String {
        match *self {
            PlayerUpgradeType::HPMaxUp => "HP Max +".to_string(),
        }
    }
}

struct PlayerUpgrade {
    upgrade_type: PlayerUpgradeType,
    level: u32,
    base_cost: u32,
    cost_per_level: u32,
    unlocked: bool,
}

impl PlayerUpgrade {
    fn cost(&self) -> u32 {
        return self.base_cost + (self.level * self.cost_per_level);
    }

    fn upgrade_name(&self) -> String {
        match self.upgrade_type {
            PlayerUpgradeType::HPMaxUp => "HP Max +".to_string(),
        }
    }

    fn upgrade_type(&self) -> PlayerUpgradeType {
        match self.upgrade_type {
            PlayerUpgradeType::HPMaxUp => PlayerUpgradeType::HPMaxUp,
        }
    }
}

struct Player {
    name: String,
    stats: PlayerStats, // track for posterity
    net_stats: HashMap<Networks, NetStats>,
    skills: Skills, // skills for checks and such
    hp: CappedValue,
    ram: CappedValue,
    credits: i32,
    xp: i32,
    upgrades: HashMap<PlayerUpgradeType, PlayerUpgrade>,
}

const BASE_SKILL_POINTS: i32 = 10;

impl Player {
    fn available_skill_points(&self) -> i32 {
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
                unlocked: false,
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

enum CappedValueType {
    Health,
    Ram,
}

struct CappedValue {
    value: i32,
    upper_limit: i32,
    value_type: CappedValueType,
}

impl CappedValue {
    fn new(value: i32, upper_limit: i32, value_type: CappedValueType) -> Self {
        Self {
            value: value,
            upper_limit: upper_limit,
            value_type: value_type,
        }
    }

    fn new_health(value: i32) -> Self {
        Self {
            value: value,
            upper_limit: value,
            value_type: CappedValueType::Health,
        }
    }

    fn new_ram(value: i32, upper_limit: i32) -> Self {
        Self {
            value: value,
            upper_limit: value,
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

    fn change_by(&mut self, amount: i32) {
        self.value = self.upper_limit.min((self.value + amount).max(0));
        if self.value == 0 {
            self.hit_zero();
        }
    }
}

struct Skills {
    hacking: i32,
    firewall: i32,
}

impl Skills {
    fn total_points(&self) -> i32 {
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
enum Networks {
    Internet,
    SIPRnet,
}

impl Networks {
    fn difficulty(&self) -> f32 {
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

#[derive(PartialEq)]
enum Tasks {
    Search,
    Datamine, // collect data from net
              // Social,   // level up social skill?
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

#[derive(Clone, Copy)]
enum InteractionType {
    BasicShop,
    AdvancedShop,
}

enum GameState {
    FreeRoam,
    Combat,
    Interacting(InteractionType),
}

struct NetrunnerGame {
    player: Player,
    state: GameState,
    terminal_lines: Vec<String>,
    current_net: Networks,
    current_task: Tasks,
    turn: i32,
    last_frame_time: Instant,
    contacts: Vec<Contact>,
}

impl Default for NetrunnerGame {
    fn default() -> Self {
        Self {
            player: Player::default(),
            state: GameState::FreeRoam,
            terminal_lines: vec![
                "welcome to cybergame".to_string(),
                "strap in, choomba".to_string(),
            ],
            current_net: Networks::Internet,
            current_task: Tasks::Datamine,
            turn: 1,
            last_frame_time: Instant::now(),
            contacts: Vec::new(),
        }
    }
}

impl NetrunnerGame {
    fn combat_attack(&mut self) {
        self.turn += 1;
        let mut dead_hostiles = vec![];
        let mut print_lines = vec![];
        for (index, contact) in self.contacts.iter_mut().enumerate() {
            // dmg to hostile
            let min_dmg_to_hostile =
                ((2 * self.player.skills.hacking) - contact.skills.firewall).max(0);
            let max_dmg_to_hostile =
                ((4 * self.player.skills.hacking) - (contact.skills.firewall / 2)).max(1);
            let dmg_to_hostile =
                rand::thread_rng().gen_range(min_dmg_to_hostile..max_dmg_to_hostile);
            // dmg to player
            let min_dmg_to_player =
                (2 + contact.skills.hacking - self.player.skills.firewall).max(0);
            let max_dmg_to_player =
                (4 + contact.skills.hacking - (self.player.skills.firewall / 2)).max(1);
            let dmg_to_player = rand::thread_rng().gen_range(min_dmg_to_player..max_dmg_to_player);
            // actually apply damage
            self.player.hp.change_by(-dmg_to_player);
            contact.hp.change_by(-dmg_to_hostile);
            print_lines.push(format!(
                "You deal {} damage to {}.",
                dmg_to_hostile, contact.name
            ));
            print_lines.push(format!(
                "You take {} damage from {}.",
                dmg_to_player, contact.name
            ));
            if contact.hp.value <= 0 {
                dead_hostiles.push(index);
                self.player.stats.kills += 1;
            }
        }
        for dead_index in dead_hostiles.iter().rev() {
            // remove dead contacts
            self.contacts.remove(*dead_index);
        }
        if self.contacts.len() == 0 {
            self.state = GameState::FreeRoam;
        }
        for line in &print_lines {
            self.terminal_print(line);
        }
    }

    fn do_task(&mut self) {
        let difficulty = self.current_net.difficulty();

        match self.current_task {
            Tasks::Search => {
                self.do_task_search(difficulty);
            }
            Tasks::Datamine => {
                self.do_task_datamine(difficulty);
            } // Tasks::Social => self.terminal_print("Nah, you don't want to do that."),
        }
    }

    fn go_shopping(&mut self) {
        // gracefully transition the player into the shopping state

        // set game state
        self.state = GameState::Interacting(InteractionType::BasicShop);
        // text in terminal
        let net_name = match self.current_net {
            Networks::Internet => "the internet",
            Networks::SIPRnet => "SIPRnet",
        };
        self.terminal_print(
            format!("You see what's available for purchase on {}.", { net_name }).as_str(),
        );
    }

    fn do_task_datamine(&mut self, difficulty: f32) {
        self.turn += 1;
        let mut rng = thread_rng();
        let success_chance = 0.6;
        let roll_success: f32 = rng.gen();
        if utils::roll_encounter(1.0 - success_chance) {
            // earn credits
            let reward_amount: i32 = (roll_success * difficulty * 4.5).ceil() as i32;
            self.player.credits += reward_amount;
            self.terminal_print(
                format!(
                    "({:.1}) You found some interesting data worth {} credits",
                    success_chance, reward_amount
                )
                .as_str(),
            );
        } else {
            // regen ram
            let reward_amount: i32 = (roll_success * 12.5 + difficulty).ceil() as i32;
            self.player.ram.change_by(reward_amount);
            self.terminal_print(
                format!(
                    "({:.1}) You don't find any new data, but regenerate {} RAM",
                    { 1.0 - success_chance },
                    reward_amount
                )
                .as_str(),
            );
        }
    }

    fn do_task_search(&mut self, difficulty: f32) {
        self.turn += 1;
        let mut rng = thread_rng();
        let roll_success: f32 = rng.gen();
        let success_chance = 0.8;
        if utils::roll_encounter(1.0 - success_chance) {
            // minor good thing - search success
            let reward_amount: i32 = (roll_success * difficulty * 14.5).ceil() as i32;
            self.player.credits += reward_amount;
            self.terminal_print(
                format!(
                    "({:.1}) You found some particularly interesting data worth {} credits",
                    success_chance, reward_amount
                )
                .as_str(),
            );
        } else {
            // bad thing - encounter
            let new_contact = Contact::new_from_level(difficulty.ceil() as i32);
            let contact_name = new_contact.name.clone();
            self.contacts.push(new_contact);
            self.terminal_print(
                format!(
                    "({:.1}) You run into a nasty piece of malware - {}",
                    { 1.0 - success_chance },
                    contact_name
                )
                .as_str(),
            );
            self.state = GameState::Combat;
        }
    }

    fn terminal_print(&mut self, line: &str) {
        self.terminal_lines.push(line.to_string())
    }

    fn player_stats_table(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            // only one row: hp and ram stats
            ui.label(colored_label(
                "HP",
                self.player.hp.value,
                self.player.hp.upper_limit,
            ));
            ui.horizontal(|ui| {
                ui.separator();
                ui.label(colored_label(
                    "RAM",
                    self.player.ram.value,
                    self.player.ram.upper_limit,
                ));
                ui.separator();
                ui.label(format!("Credits: {}", self.player.credits));
            });
            ui.end_row();
        });
    }

    fn collapsible_stats_table(&mut self, ui: &mut egui::Ui) {
        let pts = self.player.available_skill_points();
        let id = ui.make_persistent_id("my_collapsing_header");
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
            .show_header(ui, |ui| {
                ui.label("Player Stats");
                if pts > 0 {
                    ui.label(format!("- {} points available", pts));
                }
            })
            .body(|ui| {
                let can_add_skills: bool = pts > 0;
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    // row: atk/def stats
                    ui.horizontal(|ui| {
                        ui.label("ATK: ");
                        ui_counter(ui, &mut self.player.skills.hacking, can_add_skills);
                    });
                    ui.horizontal(|ui| {
                        ui.separator();
                        ui.label("DEF: ");
                        ui_counter(ui, &mut self.player.skills.firewall, can_add_skills);
                    });
                    ui.end_row();
                });
            });

        // egui::CollapsingHeader::new(label.as_str()).show(ui, |ui| {
        // });
    }

    fn list_available_networks(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Network: ");
            if ui
                .add(egui::RadioButton::new(
                    self.current_net == Networks::Internet,
                    "Internet",
                ))
                .clicked()
            {
                self.current_net = Networks::Internet
            }
            if ui
                .add(egui::RadioButton::new(
                    self.current_net == Networks::SIPRnet,
                    "SIPRnet",
                ))
                .clicked()
            {
                self.current_net = Networks::SIPRnet
            }
            // ui.radio_value(&mut self.current_net, Networks::Internet, "Internet");
            // ui.radio_value(&mut self.current_net, Networks::SIPRnet, "SIPRnet");
        });
    }

    fn combat_window(&mut self, ui: &mut egui::Ui) {
        for contact in &self.contacts {
            // ui.horizontal(|ui| {
            ui.label(format!("Contact: {}", contact.name));
            ui.label(format!(
                "HP: {}/{}",
                contact.hp.value, contact.hp.upper_limit
            ));
            ui.label(format!("Disposition: {}", contact.disposition));
            // });
        }
        if ui.button("Launch Offensive Hack").clicked() {
            self.combat_attack();
        }
        if ui.button("Exit Combat").clicked() {
            self.state = GameState::FreeRoam;
            self.contacts.clear();
            self.terminal_print("You escape from combat.")
        }
    }

    fn shop_for_upgrades(&mut self, ui: &mut egui::Ui) {
        let mut available_upgrades = vec![];
        for (_, upgrade) in self.player.upgrades.iter() {
            available_upgrades.push((upgrade.upgrade_type(), upgrade.level, upgrade.cost()))
        }
        for (up_type, up_lvl, up_cost) in available_upgrades.iter() {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "'{}' level {} for {}c",
                    up_type.name(),
                    up_lvl + 1,
                    up_cost
                ));
                if ui.button("Buy it").clicked() && self.player.credits >= *up_cost as i32 {
                    self.player.credits -= *up_cost as i32;
                    self.do_upgrade_effect(up_type);
                    self.terminal_print(format!("You bought {}!", up_type.name()).as_str());
                };
            });
        }
    }

    fn do_upgrade_effect(&mut self, upgrade: &PlayerUpgradeType) {
        // increase upgrade level by 1 and apply the upgrade's effects
        self.player.upgrades.get_mut(upgrade).unwrap().level += 1;
        match upgrade {
            PlayerUpgradeType::HPMaxUp => {
                self.player.hp.upper_limit += 50;
                self.player.hp.value += 50
            }
        }
    }

    fn net_intel_bar(&mut self, ui: &mut egui::Ui) {
        let total_intel = self
            .player
            .net_stats
            .get(&self.current_net)
            .unwrap()
            .total_intel;
        let per_level_cost = 200.0 * self.current_net.difficulty();
        let intel_level = (total_intel / per_level_cost).floor();
        let progress = (total_intel % per_level_cost) / per_level_cost;
        ui.horizontal(|ui| {
            ui.label(format!(
                "Intel level: {} ({:.1}%)",
                intel_level,
                progress * 100.0
            ));
            ui.add(egui::ProgressBar::new(progress));
        });
    }

    fn list_available_tasks(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Task: ");
            ui.selectable_value(&mut self.current_task, Tasks::Datamine, "Datamine");
            ui.selectable_value(&mut self.current_task, Tasks::Search, "Search around");
        });
        ui.horizontal(|ui| {
            if ui.button("Do Task").clicked() {
                self.do_task()
            }
            if ui.button("Enter Shop").clicked() {
                // self.state = GameState::Interacting(InteractionType::BasicShop);
                self.go_shopping();
            }
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32, can_add: bool) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if can_add {
            if ui.button("+").clicked() {
                *counter += 1;
            }
        }
    });
}

fn display_terminal(ui: &mut egui::Ui, terminal_lines: &Vec<String>) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for entry in terminal_lines {
                ui.label(entry);
            }
        });
}

fn internet_tasks(ui: &mut egui::Ui, current_value: &mut Tasks) {
    ui.selectable_value(current_value, Tasks::Datamine, "Datamine");
    ui.selectable_value(current_value, Tasks::Search, "Search around");
    // ui.selectable_value(current_value, Tasks::Social, "Practice Socializing");
}

fn siprnet_tasks(ui: &mut egui::Ui, current_value: &mut Tasks) {
    ui.selectable_value(current_value, Tasks::Datamine, "Datamine");
    ui.selectable_value(current_value, Tasks::Search, "Search around");
    // ui.selectable_value(current_value, Tasks::Social, "Social Engineering");
}

fn colored_label(label_txt: &str, current_val: i32, max_val: i32) -> egui::RichText {
    // if hp < 50%, start tinting it red
    let brightness = 160;
    let hp_ratio = current_val as f32 / max_val as f32;
    let darker_tint = ((brightness as f32 * 2.0) * hp_ratio) as u8;
    let lighter_tint = brightness + ((0.5 - hp_ratio) * (255 - brightness) as f32) as u8;
    let label_color = if hp_ratio > 0.5 {
        egui::Color32::from_rgb(brightness, brightness, brightness)
    } else {
        egui::Color32::from_rgb(lighter_tint, darker_tint, darker_tint)
    };
    egui::RichText::new(format!("{}: {} / {}", label_txt, current_val, max_val)).color(label_color)
}

impl eframe::App for NetrunnerGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // set time_delta
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_frame_time);
        self.last_frame_time = current_time;
        // 1 fps minimum even if unfocused
        ctx.request_repaint_after(Duration::from_secs(1));

        // adjust intel level over time
        self.player
            .net_stats
            .get_mut(&self.current_net)
            .unwrap()
            .total_intel += delta_time.as_secs_f32() * 1.0;

        // render GUI
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.heading(format!(
                "welcome to the net - latency: {} ms",
                delta_time.as_millis()
            ));
        });
        match self.state {
            GameState::FreeRoam => {}
            GameState::Combat => {
                egui::Window::new("Combat Window").show(ctx, |ui| {
                    self.combat_window(ui);
                });
            }
            GameState::Interacting(int_type) => {
                egui::Window::new("Interaction Window").show(ctx, |ui| {
                    match int_type {
                        InteractionType::BasicShop => {
                            ui.heading("welcome to the script kiddie shop");
                            self.shop_for_upgrades(ui);
                        }

                        InteractionType::AdvancedShop => {
                            ui.heading("welcome to the elite hacker shop");
                            self.shop_for_upgrades(ui);
                        }
                    }
                    if ui.button("Exit Shop").clicked() {
                        self.state = GameState::FreeRoam;
                    };
                });
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.player.name)
                    .labelled_by(name_label.id);
            });
            self.player_stats_table(ui);
            self.collapsible_stats_table(ui);
            ui.separator();
            // list available networks
            self.list_available_networks(ui);
            match self.current_net {
                Networks::Internet => ui.label("You are browsing the public internet."),
                Networks::SIPRnet => {
                    ui.label("You are logged in to the US DoD's classified network.")
                }
            };
            self.net_intel_bar(ui);
            ui.separator();
            // list available tasks
            self.list_available_tasks(ui);

            display_terminal(ui, &self.terminal_lines);
        });
    }
}
