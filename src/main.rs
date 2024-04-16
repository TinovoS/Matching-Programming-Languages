//Structure with needed information for memory game
struct GameState{
    num_of_cards: u8,
    picked: [bool; 6],
    correct_order: [i8; 6],
    card_order: [i8; 6],
    picked_count: u8,
    pick_number: u8,
    correct_pairs: u8,
    card: i8,
    reset: bool,
}

use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    //lets initialize game status
    //------------------------------------
    let game_state = GameState{
        num_of_cards: 6,
        picked: [true; 6],
        correct_order: [-1; 6],
        card_order: [-1; 6],
        picked_count: 0,
        pick_number: 0,
        correct_pairs: 0,
        card: 0,
        reset: false,
    };
    randomize_cards(game_state);
    //------------------------------------
    env_logger::init(); // Log to stderr (if you run with RUST_LOG=debug).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add( 
                    egui::Image::new(egui::include_image!("../resources/image1.png"))
                );

                ui.image(egui::include_image!("../resources/image1.png"));
            });
        });
    }
}
fn randomize_cards(mut game_state: GameState) -> () {
    for n in 0..6 {
        game_state.card_order[n] = -1;
    }
    for n in 0..6 {
        let mut x = rand::random::<i8>() % 6;
        while !game_state.card_order.contains(&x) {
            x = rand::random::<i8>() % 6;
        }
        game_state.card_order[n] = x as i8;
        game_state.picked[n] = false;

    }
    game_state.pick_number = 0;
    game_state.correct_pairs = 0;
    game_state.card = -1;
}
