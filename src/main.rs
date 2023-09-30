#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

mod utils;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "cybergame",
        options,
        Box::new(|_cc| Box::<NetrunnerGame>::default()),
    )
}

trait HasHealth {
    fn get_hp(&self) -> i32;
    fn max_hp(&self) -> i32;
    fn set_hp(&mut self, amount: i32);

    fn hp_up(&mut self, amount: i32) {
        let healable_damage = self.max_hp() - self.get_hp();
        if healable_damage == 0 {
            return;
        }
        self.set_hp(self.max_hp().min(self.get_hp() + amount))
    }

    fn hp_down(&mut self, amount: i32) {
        if amount > self.get_hp() {
            // DEATH
            self.set_hp(0);
        } else {
            self.set_hp(self.get_hp() - amount)
        }
    }
}

trait HasRam {
    fn get_ram(&self) -> i32;
    fn max_ram(&self) -> i32;
    fn set_ram(&mut self, amount: i32);

    fn ram_up(&mut self, amount: i32) {
        let recoverable_ram = self.max_ram() - self.get_ram();
        if recoverable_ram == 0 {
            return;
        }
        self.set_ram(self.max_ram().min(self.get_ram() + amount))
    }

    fn ram_down(&mut self, amount: i32) {
        if amount > self.get_ram() {
            // DEATH
            self.set_ram(0);
        } else {
            self.set_ram(self.get_ram() - amount)
        }
    }
}

struct Player {
    name: String,
    skills: Skills,
    hp: i32,
    ram: i32,
}

impl Player {
    fn max_ram(&self) -> i32 {
        return 100;
    }

    fn available_skill_points(&self) -> i32 {
        // debug - for now, 14 points is the max
        return 14 - self.skills.total_points();
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: random_default_name(),
            skills: Skills::default(),
            hp: 100,
            ram: 50,
        }
    }
}

impl HasHealth for Player {
    fn get_hp(&self) -> i32 {
        return self.hp;
    }
    fn max_hp(&self) -> i32 {
        return 100;
    }
    fn set_hp(&mut self, amount: i32) {
        self.hp = amount.min(self.max_hp())
    }
}

impl HasRam for Player {
    fn get_ram(&self) -> i32 {
        return self.ram;
    }
    fn max_ram(&self) -> i32 {
        return 100;
    }
    fn set_ram(&mut self, amount: i32) {
        self.ram = amount.min(self.max_ram())
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
            hacking: 5,
            firewall: 5,
        }
    }
}

#[derive(PartialEq)]
enum Networks {
    Internet,
    SIPRnet,
}

#[derive(PartialEq)]
enum Tasks {
    Search,
    Datamine, // collect data from net
    Social,   // level up social skill?
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

struct NetrunnerGame {
    player: Player,
    terminal_lines: Vec<String>,
    current_net: Networks,
    current_task: Tasks,
    turn: i32,
}

impl Default for NetrunnerGame {
    fn default() -> Self {
        Self {
            player: Player::default(),
            terminal_lines: vec![
                "welcome to cybergame".to_string(),
                "strap in, choomba".to_string(),
            ],
            current_net: Networks::Internet,
            current_task: Tasks::Datamine,
            turn: 1,
        }
    }
}

impl NetrunnerGame {
    fn do_task(&mut self) {
        let difficulty = match self.current_net {
            Networks::Internet => 1.0,
            Networks::SIPRnet => 3.0,
        };

        match self.current_task {
            Tasks::Search => {
                self.do_task_search(difficulty);
            }
            Tasks::Datamine => self.terminal_print("Nah, you don't want to do that."),
            Tasks::Social => self.terminal_print("Nah, you don't want to do that."),
        }
    }

    fn do_task_search(&mut self, difficulty: f32) {
        let mut rng = thread_rng();
        let net_name = match self.current_net {
            Networks::Internet => "the internet",
            Networks::SIPRnet => "SIPRnet",
        };
        self.terminal_print(
            format!(
                "You search {} for any interesting information you can find.",
                { net_name }
            )
            .as_str(),
        );
        //
        let roll_success: f32 = rng.gen();
        let success_chance = 0.8;
        if utils::roll_encounter(1.0 - success_chance) {
            // minor good thing - search success
            let reward_amount: i32 = (roll_success * difficulty * 3.0).ceil() as i32;
            self.player.ram_up(reward_amount);
            self.terminal_print(
                format!("You found some interesting data. You gained {} RAM", {
                    reward_amount
                })
                .as_str(),
            );
        } else {
            // bad thing - encounter
            let damage_amount = (roll_success * difficulty * 2.0).floor() as i32;
            self.player.hp_down(damage_amount);
            self.terminal_print(
                format!(
                    "You run into a nasty piece of malware that does {} damage.",
                    { damage_amount }
                )
                .as_str(),
            );
        }
    }

    fn terminal_print(&mut self, line: &str) {
        self.terminal_lines.push(line.to_string())
    }

    fn player_stats_table(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            // row: hp and ram stats
            ui.label(colored_label("HP", self.player.hp, self.player.max_hp()));
            ui.horizontal(|ui| {
                ui.separator();
                ui.label(colored_label("RAM", self.player.ram, self.player.max_ram()));
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
    ui.selectable_value(current_value, Tasks::Social, "Practice Socializing");
}

fn siprnet_tasks(ui: &mut egui::Ui, current_value: &mut Tasks) {
    ui.selectable_value(current_value, Tasks::Datamine, "Datamine");
    ui.selectable_value(current_value, Tasks::Search, "Search around");
    ui.selectable_value(current_value, Tasks::Social, "Social Engineering");
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("welcome to the net");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.player.name)
                    .labelled_by(name_label.id);
            });
            self.player_stats_table(ui);
            ui.separator();
            self.collapsible_stats_table(ui);
            if ui.button("take dmg").clicked() {
                self.player.hp_down(10);
                self.terminal_print("Ouch!");
            }
            // list available networks
            self.list_available_networks(ui);
            // list available tasks
            ui.horizontal(|ui| {
                ui.label("Task: ");
                match self.current_net {
                    Networks::Internet => internet_tasks(ui, &mut self.current_task),
                    Networks::SIPRnet => {
                        siprnet_tasks(ui, &mut self.current_task);
                    }
                }
            });
            match self.current_net {
                Networks::Internet => ui.label("You are browsing the public internet."),
                Networks::SIPRnet => {
                    ui.label("You are logged in to the US DoD's classified network.")
                }
            };
            if ui.button("Go").clicked() {
                self.do_task()
            }
            ui.separator();
            display_terminal(ui, &self.terminal_lines);
        });
    }
}
