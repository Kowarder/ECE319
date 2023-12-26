use eframe::{egui, run_native, NativeOptions};
use rfd::FileDialog;

struct MyApp {
    data: String, // This will store the file data
    simulation_parameter: f32,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            simulation_parameter: 0.0,
        }
    }

    fn load_file_contents(&mut self) {
        if let Some(file) = FileDialog::new().pick_file() {
            match std::fs::read_to_string(file) {
                Ok(contents) => {
                    self.data = contents;
                }
                Err(e) => {
                    self.data = format!("Error reading file: {}", e);
                }
            }
        }
    }
    
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Simulation Control");

            if ui.button("Run Simulation").clicked() {
                // Trigger simulation
            }

            ui.add(egui::Slider::new(&mut self.simulation_parameter, 0.0..=100.0).text("Parameter"));

            if ui.button("Load Data File").clicked() {
                self.load_file_contents();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simulation Data");
            ui.label(&self.data); // Display file data here
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native(
        "My Simulation App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}
