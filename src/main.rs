use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1200.0]),
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let num_columns = 4;
        let num_rows = 3;
        egui::CentralPanel::default().show(ctx, |ui| {

            let available_size = ui.available_size();
            let min_col_width = available_size.x*0.99  / num_columns as f32;
            let min_row_height = available_size.y*0.99  / num_rows as f32;
            for _ in 0..num_rows  {
            egui::Grid::new(num_rows)
                .num_columns(num_columns)
                .min_col_width(min_col_width)
                .min_row_height(min_row_height)
                .show(ui, |mut row| {
                    for _ in 0..num_columns  { // Add 4 image buttons
                        row.add(
                            egui::ImageButton::new(
                                egui::include_image!("/home/labus/Desktop/Matching-Programming-Languages/resources/background.svg")
                            )
                            
                        );
                    }
                });
            }
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