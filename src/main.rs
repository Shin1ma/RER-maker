use eframe::egui::{self};

fn main() -> eframe::Result {
    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400f32, 300f32]),
        ..Default::default()
    };

    eframe::run_native(
        "ER maker",
        window_options,
        Box::new(|_ctx| Ok(Box::new(ErMakerApp::default()))),
    )
}

struct ErMakerApp {
    name: String,
}

impl ErMakerApp {
    fn new(name: &str) -> Self {
        ErMakerApp {
            name: name.to_string(),
        }
    }
}
impl Default for ErMakerApp {
    fn default() -> Self {
        ErMakerApp {
            name: "Jonh".to_string(),
        }
    }
}
impl eframe::App for ErMakerApp {
    fn ui(&mut self, mut ui: &mut egui::Ui, mut _frame: &mut eframe::Frame) {
        ui.heading(format!("this is my app: {}", self.name));
        ui.label("hi there");
    }
}
