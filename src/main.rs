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
    ram: i32,
}

impl Player {
    fn max_ram(&self) -> i32 {
        return 100;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Anonymous".to_string(),
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
    ui.selectable_value(current_value, Tasks::SearchTasks, "Search around");
    ui.selectable_value(current_value, Tasks::SocialPractice, "Socialize");
}

fn siprnet_tasks(ui: &mut egui::Ui, current_value: &mut Tasks) {
    ui.selectable_value(current_value, Tasks::SearchTasks, "Search around");
    ui.selectable_value(current_value, Tasks::Datamine, "Datamine");
}

fn colored_label(label_txt: &str, current_val: i32, max_val: i32) -> egui::RichText {
    // if hp < 50%, start tinting it red
    let hp_ratio = current_val as f32 / max_val as f32;
    let tint = ((255 * 2) as f32 * hp_ratio) as u8;
    let label_color = if hp_ratio > 0.5 {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_rgb(255, tint, tint)
    };
    egui::RichText::new(format!("{}: {} / {}", label_txt, current_val, max_val)).color(label_color)
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
                let can_add_skills = self.player.skills.total_points() < 14;
                ui.label("ATK: ");
                ui_counter(ui, &mut self.player.skills.hacking, can_add_skills);
                ui.separator();
                ui.label("DEF: ");
                ui_counter(ui, &mut self.player.skills.firewall, can_add_skills);
            });
            ui.horizontal(|ui| {
                ui.label(colored_label("HP", self.player.hp, self.player.max_hp()));
                ui.separator();
                ui.label(colored_label("RAM", self.player.ram, self.player.max_ram()));
                // ui.add(egui::widgets::ProgressBar::new(
                //     self.player.hp as f32 / self.player.max_hp() as f32,
                // ));
            });
            ui.horizontal(|ui| {
                // ui.label(
                //     egui::RichText::new(format!(
                //         "RAM: {} / {}",
                //         self.player.ram,
                //         self.player.max_ram()
                //     ))
                //     .color(egui::Color32::RED),
                // );
                // ui.add(egui::widgets::ProgressBar::new(
                //     self.player.ram as f32 / self.player.max_ram() as f32,
                // ));
            });
            if ui.button("+").clicked() {
                self.player.hp_down(10);
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
                self.terminal_print("thanks bud");
            }
            ui.separator();
            display_terminal(ui, &self.terminal_lines);
        });
    }
}
