use std::collections::HashMap;

use chrono::{Local, NaiveDate};
use egui_extras::DatePickerButton;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    add_new_list: bool,
    list_namager: ListManager,
    new_list_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct ListManager {
    todo_lists: HashMap<String, TodoList>,
    current_list_name: String,
    add_new_list: bool,
}

impl ListManager {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.button("All List").clicked() {
            self.add_new_list = true;
        }

        if self.add_new_list {
            egui::Window::new("Add New List").show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.current_list_name);
                if ui.button("Add List").clicked() && !self.current_list_name.is_empty() {
                    let list_name = std::mem::take(&mut self.current_list_name);
                    self.todo_lists.insert(
                        list_name.clone(),
                        TodoList {
                            list_name,
                            items: Vec::new(),
                            current_item_name: String::new(),
                            current_list_date: NaiveDate::default(),
                        },
                    );
                    self.add_new_list = false;
                }
            });
        }

        ui.horizontal(|ui| {
            ui.separator();
            for lists in self.todo_lists.iter_mut() {
                let name = lists.0;
                let list = lists.1;
                egui::Window::new(name).show(ctx, |ui| {
                    list.show(ui);
                });
            }
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ListItem {
    name: String,
    completed: bool,
    date: NaiveDate,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct TodoList {
    list_name: String,
    items: Vec<ListItem>,
    current_item_name: String,
    current_list_date: NaiveDate,
}

impl TodoList {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        for item in self.items.iter_mut() {
            if ui.radio(item.completed, &item.name).clicked() {
                item.completed = !item.completed;
            }
            ui.separator();
            ui.label(item.date.to_string());
        }

        ui.text_edit_singleline(&mut self.current_item_name);
        ui.add(DatePickerButton::new(&mut self.current_list_date));

        if ui.button("Add Item").clicked() {
            let name = std::mem::take(&mut self.current_item_name);
            self.items.push(ListItem {
                name,
                completed: false,
                date: self.current_list_date.clone()
            });
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Rudium Planner");

            self.list_namager.show(ctx, ui);

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
