use egui::{SliderOrientation, Ui, Vec2};
use egui::style::Spacing;

use std::fs::File;
use std::io::Write;
use crate::ConnectedUser;


pub struct TemplateApp {

    users: Vec<ConnectedUser>

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            users: Vec::new()
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>, users:Vec<ConnectedUser>) -> Self {

        TemplateApp{
            users,
        }
    }
}

impl eframe::App for TemplateApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            users,
        } = self;

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.spacing_mut().slider_width = 200.0;
            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.label("CONNECTED USERS:\n");

            for i in 0..self.users.len() {
                ui.label(format!("{}   (id={}) ", self.users[i].name, self.users[i].id,));
                if ui.button("^ disable user ^").clicked() {
                    self.users.remove(i);
                }
            }
        });
    }
}