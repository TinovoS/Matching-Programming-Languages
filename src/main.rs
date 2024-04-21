use eframe::egui;
use egui::ImageData;
use std::path::Path;
const NUM_OF_CARDS: usize = 12;

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
            Box::<GameState>::default()
        }),
    )
}

#[derive(Default)]  
struct GameState {
    picked: [bool; 12 ],
    correct_order: [i8; 12 ],
    card_order: [i8; 12 ],
    card_picture: [&'static str; 12],
    picked_count: u8,
    pick_number: u8,
    correct_pairs: u8,
    card: i8,
    reset: bool,

}

    
impl GameState {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut game_state = GameState {
            picked: [true; NUM_OF_CARDS],
            correct_order: [-1; NUM_OF_CARDS],
            card_order: [-1; NUM_OF_CARDS],
            card_picture: [""; NUM_OF_CARDS],
            picked_count: 0,
            pick_number: 0,
            correct_pairs: 0,
            card: 0,
            reset: false,
        };

        // Initialize the state
        randomize_cards(&mut game_state);

        // Return the initialized GameState
        game_state
    }
}
impl eframe::App for GameState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let num_columns = 4;
        let num_rows = 3;
        //lets initialize game status
        //------------------------------------
        //------------------------------------
        egui::CentralPanel::default().show(ctx, |ui| {

            let available_size = ui.available_size();
            let min_col_width = available_size.x*0.99  / num_columns as f32;
            let min_row_height = available_size.y*0.99  / num_rows as f32;

            for n in 0..num_rows  {
            egui::Grid::new(num_rows)
                .num_columns(num_columns)
                .min_col_width(min_col_width)
                .min_row_height(min_row_height)
                .show(ui, |mut row| {
                    for m in 0..num_columns  { // Add 4 image buttons
                        //inside my update function

                        let img = row.ctx().load_texture(
                            "my-image",
                            get_image(&self.card_picture[n * num_columns + m], 0, 0, 100, 100),
                            Default::default()
                        );

                        row.add(
                            
                             
                            egui::ImageButton::new(&img)
                        
                        
                        );
                        /*
                        row.add(
                            egui::ImageButton::new(
                                egui::include_image!(game_state.card_picture[n*num_rows+m])
                            )
                        );*/
                    }
                });
            }
        });
    }
}

fn randomize_cards(mut game_state: &mut GameState) -> () {
    for n in 0..12 {
        game_state.card_order[n] = -1;
    }
    for n in 0..12 {
        let mut x = (rand::random::<i8>()).rem_euclid(12);
        while game_state.card_order.contains(&x) {
            x = (rand::random::<i8>()).rem_euclid(12);
        }
        game_state.card_order[n] = x;
        game_state.picked[n] = false;
    }
    game_state.card_picture[game_state.card_order[0] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[1] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[2] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[3] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[4] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[5] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[6] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[7] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[8] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[9] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[10] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[11] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";

    game_state.pick_number = 0;
    game_state.correct_pairs = 0;
    game_state.card = -1;
    let num_columns = 4;
    let num_rows = 3;
    for n in 0..num_rows {
        for m in 0..num_columns { // Add 4 image buttons
            println!("n {} o {}, p {}", n * num_columns + m, (game_state.card_order[n * num_columns + m] as u8) as usize, game_state.card_picture[n * num_columns + m]);
        }
    }
}

pub fn get_image(filepath: &str, ix: u32, iy: u32, iw: u32, ih: u32) -> ImageData {
    let fp = Path::new(filepath);
    let color_image = load_image_from_path(&fp).unwrap();
    let img = ImageData::from(color_image);
    img
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
