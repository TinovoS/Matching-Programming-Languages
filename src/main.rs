mod game_state;
use eframe::egui;
use game_state::GameState;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_fullscreen(true)
        .with_title("Matching Programming Languages"),
        ..Default::default()
    };
    eframe::run_native(
        "My parallel egui App",
        options,
        Box::new(|_cc| Box::new(GameState::new())),
    )
}

