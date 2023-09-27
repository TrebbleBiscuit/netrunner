#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "cybergame",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
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

struct Player {
    name: String,
    skills: Skills,
    hp: i32,
    xp: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
            skills: Skills::default(),
            hp: 100,
            xp: 0,
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

struct Skills {
    hacking: i32,
    firewall: i32,
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
    SearchTasks,
    Datamine,       // collect data from net
    SocialPractice, // level up social skill?
}

struct MyApp {
    player: Player,
    name: String,
    terminal_lines: Vec<String>,
    current_net: Networks,
    current_task: Tasks,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            player: Player::default(),
            name: "Cyberpunk".to_owned(),
            terminal_lines: vec![
                "welcome to cybergame".to_string(),
                "strap in, choomba".to_string(),
            ],
            current_net: Networks::Internet,
            current_task: Tasks::Datamine,
        }
    }
}

impl MyApp {
    fn terminal_print(&mut self, line: &str) {
        self.terminal_lines.push(line.to_string())
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("wake up cyberman");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                ui.label("ATK: ");
                ui_counter(ui, &mut self.player.skills.hacking);
            });
            ui.horizontal(|ui| {
                ui.label("DEF: ");
                ui_counter(ui, &mut self.player.skills.firewall);
            });
            // ui.label(format!("HP: {}", self.player.hp));
            ui.horizontal(|ui| {
                ui.label(format!("HP: {} / {}", self.player.hp, self.player.max_hp()));
                ui.add(egui::widgets::ProgressBar::new(
                    self.player.hp as f32 / self.player.max_hp() as f32,
                ));
            });
            if ui.button("+").clicked() {
                self.terminal_print("thanks bud");
                // self.terminal_lines.push("thanks bud".to_string())
            }
            // list available networks
            ui.horizontal(|ui| {
                ui.label("Network: ");
                ui.radio_value(&mut self.current_net, Networks::Internet, "Internet");
                ui.radio_value(&mut self.current_net, Networks::SIPRnet, "SIPRnet");
            });
            // list available tasks
            ui.horizontal(|ui| {
                ui.label("Task: ");
                match self.current_net {
                    Networks::Internet => {
                        ui.radio_value(
                            &mut self.current_task,
                            Tasks::SearchTasks,
                            "Search for ..?",
                        );
                        ui.radio_value(&mut self.current_task, Tasks::SocialPractice, "Socialize");
                    }
                    Networks::SIPRnet => {
                        ui.radio_value(
                            &mut self.current_task,
                            Tasks::SearchTasks,
                            "Search for ..?",
                        );
                        ui.radio_value(&mut self.current_task, Tasks::Datamine, "Datamine");
                    }
                }
            });
            match self.current_net {
                Networks::Internet => ui.label("You are browsing the public internet."),
                Networks::SIPRnet => {
                    ui.label("You are logged in to the US DoD's classified network.")
                }
            };
            ui.separator();
            display_terminal(ui, &self.terminal_lines);
        });
    }
}
