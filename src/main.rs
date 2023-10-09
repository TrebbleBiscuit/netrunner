#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Color32, RichText};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

mod buffs;
mod pieces;
mod player;
mod utils;

use pieces::{Contact, Networks};
use player::{Player, PlayerFlag, PlayerUpgradeType};
use utils::roll_encounter;

// update at this framerate when there is no user input
const MAX_WAIT_BETWEEN_FRAMES: Duration = Duration::from_millis(200); // 200ms = 5 fps

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(450.0, 400.0)),
        always_on_top: true,
        ..Default::default()
    };
    eframe::run_native(
        "cybergame",
        options,
        Box::new(|_cc| Box::<NetrunnerGame>::default()),
    )
}

#[derive(PartialEq)]
enum Tasks {
    Search,
    Datamine, // collect data from net
              // Social,   // level up social skill?
}

impl Tasks {
    fn description(&self) -> RichText {
        match *self {
            Tasks::Search => RichText::new("High risk, ++ Credits, ???").color(Color32::DARK_GRAY),
            Tasks::Datamine => {
                RichText::new("Low risk, + Credits, + Ram").color(Color32::DARK_GRAY)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum InteractionType {
    BasicShop,
    AdvancedShop,
}

#[derive(PartialEq)]
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
    fn do_turn(&mut self) {
        self.turn += 1;
        self.player.buffs.do_turn();
    }

    fn combat_attack(&mut self) {
        let mut dead_hostiles = vec![];
        let mut print_lines = vec![];
        for (index, contact) in self.contacts.iter_mut().enumerate() {
            // dmg to hostile
            let min_dmg_to_hostile =
                ((2 * self.player.skills.hacking) - contact.skills.security).max(0);
            let max_dmg_to_hostile =
                ((4 * self.player.skills.hacking) - (contact.skills.security / 2)).max(1);
            let dmg_to_hostile =
                rand::thread_rng().gen_range(min_dmg_to_hostile..max_dmg_to_hostile);
            // buff dmg
            let buff_dmg = self.player.buffs.get_buff_dmg(dmg_to_hostile);
            let buff_text = if buff_dmg > 0 {
                format!(" + {}", buff_dmg)
            } else {
                String::new()
            };
            // dmg to player
            let min_dmg_to_player =
                (2 + contact.skills.hacking - self.player.skills.security).max(0);
            let max_dmg_to_player =
                (4 + contact.skills.hacking - (self.player.skills.security / 2)).max(1);
            let dmg_to_player = rand::thread_rng().gen_range(min_dmg_to_player..max_dmg_to_player);
            // actually apply damage
            self.player.hp.change_by(-dmg_to_player);
            contact.hp.change_by(-(dmg_to_hostile + buff_dmg));
            print_lines.push(format!(
                "You deal {}{} damage to {}.",
                dmg_to_hostile, buff_text, contact.name
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

    fn ability_overclock(&mut self) {
        self.terminal_print("You overclock your systems, empowering your next attack.");
        self.player.buffs.add_buff(buffs::BuffType::Overclock, 1);
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
        self.do_turn();
        let mut rng = thread_rng();
        let success_chance = 0.6;
        let roll_success: f32 = rng.gen();
        if roll_encounter(1.0 - success_chance) {
            // success - earn credits
            self.player.stats.datamine_success += 1;
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
            // "fail" - regen ram
            let reward_amount: i32 = (roll_success * 14.5 + difficulty).ceil() as i32;
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
        self.do_turn();
        let mut rng = thread_rng();
        let roll_success: f32 = rng.gen();

        // first-time encounters
        match self.current_net {
            Networks::Internet => {
                if !self.player.has_flag(&PlayerFlag::DiscoveredShopBasic)
                    && self.player.credits >= 100
                {
                    if roll_encounter(0.2) {
                        // stumble across the shop
                        self.terminal_print(
                        "You stumble across some sort of virtual server for secure transations.",
                    );
                        self.go_shopping();
                        return;
                    }
                }
            }
            Networks::SIPRnet => {}
        }

        let success_chance = 0.8;
        if roll_encounter(1.0 - success_chance) {
            // minor good thing - search success
            self.player.stats.search_success += 1;
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
            let new_contact = Contact::new(difficulty.ceil() as i32, &self.current_net);
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
                    // ui.label(format!("- {} points available", pts));
                    ui.label(
                        RichText::new(format!("- {} point{} available!", pts, {
                            if pts > 1 {
                                "s"
                            } else {
                                ""
                            }
                        }))
                        .color(egui::Color32::LIGHT_GRAY),
                    );
                }
            })
            .body(|ui| {
                let enabled = match self.state {
                    GameState::FreeRoam => true,
                    _ => false,
                };
                ui.add_enabled_ui(enabled, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        // row: atk/def stats
                        ui.horizontal(|ui| {
                            ui.label("Hacking: ")
                                .on_hover_text("Increases attack damage");
                            ui_counter(ui, &mut self.player.skills.hacking);
                        });
                        ui.horizontal(|ui| {
                            ui.separator();
                            ui.label("Security: ")
                                .on_hover_text("Mitigates enemy hacks");
                            ui_counter(ui, &mut self.player.skills.security);
                        });
                        ui.end_row();
                    });
                });
            });

        // egui::CollapsingHeader::new(label.as_str()).show(ui, |ui| {
        // });
    }

    fn list_available_networks(&mut self, ui: &mut egui::Ui) {
        // you can only change networks in free roam
        let enabled = match self.state {
            GameState::FreeRoam => true,
            _ => false,
        };
        ui.add_enabled_ui(enabled, |ui| {
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
                    // TODO: first time login
                }
                // ui.radio_value(&mut self.current_net, Networks::Internet, "Internet");
                // ui.radio_value(&mut self.current_net, Networks::SIPRnet, "SIPRnet");
            });
        });
    }

    fn combat_window(&mut self, ui: &mut egui::Ui) {
        for contact in &self.contacts {
            // ui.horizontal(|ui| {
            ui.heading(RichText::new("Threat Detected").color(Color32::from_rgb(200, 100, 0)));
            ui.label(format!("Contact: {}", contact.name));
            ui.label(colored_label(
                "HP",
                contact.hp.value,
                contact.hp.upper_limit,
            ));
            ui.label(format!("Disposition: {}", contact.disposition));
            // });
        }
        ui.horizontal(|ui| {
            let attack_cost = 4;
            if ui.button("Launch Hack").clicked() {
                if self.player.ram.value >= attack_cost {
                    self.player.ram.change_by(-attack_cost);
                    self.combat_attack();
                    self.do_turn();
                } else {
                    self.terminal_print(
                        format!("You need {} RAM to use that ability.", attack_cost).as_str(),
                    )
                }
            }
            let overclock_cost = 10;
            if ui.button("Overclock Systems").clicked() {
                if self.player.ram.value >= overclock_cost {
                    self.player.ram.change_by(-overclock_cost);
                    self.ability_overclock();
                } else {
                    self.terminal_print(
                        format!("You need {} RAM to use that ability.", overclock_cost).as_str(),
                    )
                }
            }
            if ui
                .button(RichText::new("Escape Combat").color(Color32::GRAY))
                .clicked()
            {
                self.state = GameState::FreeRoam;
                self.contacts.clear();
                self.terminal_print("You escape from combat.")
            }
        });
    }

    fn shop_for_upgrades(&mut self, ui: &mut egui::Ui) {
        if self.player.has_flag(&PlayerFlag::DiscoveredShopBasic) {
            ui.label("New here? Don't recognize you. c Let's do biz.");
        };
        let mut available_upgrades = vec![];
        for (_, upgrade) in self.player.upgrades.iter() {
            if upgrade.available {
                available_upgrades.push((
                    upgrade.upgrade_type.clone(),
                    upgrade.level,
                    upgrade.cost(),
                ))
            }
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
            PlayerUpgradeType::SecurityUp => todo!(),
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
        ui.heading("Task selection");
        ui.horizontal(|ui| {
            ui.label("Task: ");
            ui.selectable_value(&mut self.current_task, Tasks::Datamine, "Datamine");
            ui.selectable_value(&mut self.current_task, Tasks::Search, "Search around");
        });
        ui.label(self.current_task.description());
        ui.horizontal(|ui| {
            if ui.button("Do Task").clicked() {
                self.do_task()
            }
            if self.player.has_flag(&PlayerFlag::DiscoveredShopBasic) {
                if ui
                    .button(RichText::new("Enter Shop").color(Color32::GRAY))
                    .clicked()
                {
                    // self.state = GameState::Interacting(InteractionType::BasicShop);
                    self.go_shopping();
                }
            };
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
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

fn colored_label(label_txt: &str, current_val: i32, max_val: i32) -> RichText {
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
    RichText::new(format!("{}: {} / {}", label_txt, current_val, max_val)).color(label_color)
}

impl eframe::App for NetrunnerGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // set time_delta
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_frame_time);
        self.last_frame_time = current_time;
        // 1 fps minimum even if unfocused
        ctx.request_repaint_after(MAX_WAIT_BETWEEN_FRAMES);

        // adjust intel level over time
        self.player
            .net_stats
            .get_mut(&self.current_net)
            .unwrap()
            .total_intel += delta_time.as_secs_f32() * 1.0;

        // render GUI
        let browse_flavor_txt = match self.state {
            GameState::Combat => "Engaged in combat",
            _ => "Cruising the net",
        };
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.heading(format!(
                "{} - latency: {} ms",
                browse_flavor_txt,
                delta_time.as_millis()
            ));
        });

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
            match self.state {
                GameState::FreeRoam => self.list_available_tasks(ui),
                GameState::Combat => self.combat_window(ui),
                GameState::Interacting(int_type) => match int_type {
                    InteractionType::BasicShop => {
                        ui.heading("welcome to the script kiddie shop");
                        self.shop_for_upgrades(ui);
                        if ui.button("Exit Shop").clicked() {
                            self.state = GameState::FreeRoam;
                            self.player.enable_flag(PlayerFlag::DiscoveredShopBasic)
                        };
                    }

                    InteractionType::AdvancedShop => {
                        ui.heading("welcome to the elite hacker shop");
                        self.shop_for_upgrades(ui);
                        if ui.button("Exit Shop").clicked() {
                            self.state = GameState::FreeRoam;
                            self.player.enable_flag(PlayerFlag::DiscoveredShopBasic)
                        };
                    }
                },
            }
            ui.separator();
            display_terminal(ui, &self.terminal_lines);
        });
    }
}
