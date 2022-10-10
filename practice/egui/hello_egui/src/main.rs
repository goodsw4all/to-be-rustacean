use eframe::{
    egui::{self, RichText},
    epaint::{Color32, FontId},
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.label("Top World!");
        });
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Side World!");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.vertical(|ui| {
                    ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(
                egui::Slider::new(&mut self.age, 0..=120)
                    .text("age")
                    .show_value(true),
            );
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label(
                RichText::new("Large text")
                    .font(FontId::proportional(40.0))
                    .color(Color32::RED),
            );
        });
    }
}
