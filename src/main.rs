use eframe::egui;

#[derive(Default)]
struct GjallarbruApp {
    // Your state here...
}

impl GjallarbruApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GjallarbruApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Frame can be used to paint custom graphics:
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gjallarbru");
            ui.label("Welcome to Gjallarbru!");
            ui.label("This is a work in progress.");
            ui.label("Please check back later.");
        });
    }
}
fn main() {
    print!("Hello, world!");
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gjallarbru",
        native_options,
        Box::new(|cc| Box::new(GjallarbruApp::new(cc))),
    );
}
