use eframe::egui::{self, include_image};
use egui::ImageData;
use std::path::Path;
use std::{thread, time};
use egui::{RichText, FontId, Color32};

const NUM_OF_CARDS: usize = 12;


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

#[derive(Default)]  
struct GameState {
    picked_globaly: [bool; 12 ],
    picked_localy: [bool; 12 ],
    card_order: [i8; 12 ],
    card_picture: [&'static str; 12],
    picked_count: u8,
    pick_number: u8,
    correct_pairs: u8,
    card: i8,
    reset: bool,
    background_pictures: [&'static str; 12],
    background_picture: &'static str,
    exit_picture: &'static str,

}

    
impl GameState {
    /// Called once before the first frame.
     fn new() -> Self {
        // Initialize the struct fields here
        let mut game_state = Self {
            picked_localy: [false; NUM_OF_CARDS], //picked check which one is picked
            picked_globaly: [false; NUM_OF_CARDS],
            card_order: [-1; NUM_OF_CARDS], //card order used in randomize
            card_picture: [""; NUM_OF_CARDS],//card picture literally picture
            picked_count: 0, // count how many cards are picked
            pick_number: 0, 
            correct_pairs: 0, //counts how many pairs are correct
            card: 0,
            reset: false,
            background_pictures: [""; NUM_OF_CARDS],
            background_picture: "",
            exit_picture: "",
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
                let min_row_height = available_size.y*0.80  / num_rows as f32;


                //here we represent labels
                egui::Grid::new("header")
                    .num_columns(3)
                    .min_col_width(available_size.x*0.99  / 3.0)
                    //.min_row_height(min_row_height)
                    //.max_col_width(available_size.x*0.99  / 3.0)
                    .show(ui, |mut row| {
                        let exit_img = row.ctx().load_texture(
                            "my-image",
                            get_image(&self.exit_picture, 0, 0, 100, 100),
                            Default::default(),
                        );
                        let exit_button = egui::ImageButton::new(&exit_img);
                        if row.add(exit_button).clicked() {
                            row.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                
                        // Second column: Text label with bordered frame
                       

                        const GRAY: egui::Color32 = egui::Color32::from_rgb(38, 38, 38);
                        

                        let framed_text_label = egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(200.0,0.0))
                        .fill(GRAY)
                        .show(row, |ui| {
                            ui.label(RichText::new(self.correct_pairs.to_string()).font(FontId::proportional(40.0)));
                        });

                        // Third column: Another text label with bordered frame
                        let another_text_label = "Another Text Here";
                        let framed_another_text_label = egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(200.0,0.0))
                        .fill(GRAY)
                        .show(row, |ui| {
                            ui.label(RichText::new((6-self.correct_pairs).to_string()).font(FontId::proportional(40.0)));
                        });
                });

                
                for n in 0..num_rows  {
                egui::Grid::new(n)
                    .num_columns(num_columns)
                    .min_col_width(min_col_width)
                    .min_row_height(min_row_height)
                    .max_col_width(min_col_width)
                    .show(ui, |mut row| {
                        for m in 0..num_columns  { // Add 4 image buttons
                            //inside my update function
                        
                            let background_img = row.ctx().load_texture(
                                "my-image",
                                get_image(&self.background_pictures[n * num_columns + m], 0, 0, 100, 100),
                                Default::default(),
                            );
                            if self.picked_globaly[n * num_columns + m] {
                                row.add(egui::Image::new(&background_img));
                            }
                            else {
                            if row.add(egui::ImageButton::new(&background_img)).clicked()  {
                                if self.picked_count==2{
                                    //Here compare cards also reset cards to original
                                    compare_cards(self);
                                    reset_background_cards(self)
                                }
                                self.background_pictures[n * num_columns + m] = self.card_picture[n * num_columns + m];
                                self.picked_localy[n * num_columns + m] = true;
                                self.picked_globaly[n * num_columns + m] = true;
                                self.picked_count +=1;
                            }
                        }
                        }
                    });
                }
            });
        }
    }

    fn compare_cards(game_state: &mut GameState) -> () {
        let mut buffer: [&'static str; 2] = [""; 2];
        let mut k = 0;
        for i in 0..12 {
            if game_state.picked_localy[i] {
                buffer[k] = game_state.background_pictures[i];
                k +=1;
            }
        }


        if buffer[0] == buffer[1]{
            game_state.correct_pairs +=1;

        }else {
            for i in 0..12 {
                if game_state.picked_localy[i]{
                    game_state.picked_globaly[i] = false;
                }
            }

        }
        // TODO
        //ovde sad treba da se obrisu dva
        // mozemo da stavimo da ne nestaju

    }

    fn reset_background_cards(game_state: &mut GameState) {
        // Reset picked count
        game_state.picked_count = 0;

        for i in 0..12 {
            game_state.picked_localy[i] =false;
        }
        // Calculate the end time
        thread::sleep(time::Duration::from_secs(1));

            for i in 0..12 {
                if game_state.picked_globaly[i] == false {
                        game_state.background_pictures[i as usize] =
                            "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/background.png";
                }
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
        game_state.picked_localy[n] = false;
    }
    game_state.card_picture[game_state.card_order[0] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[1] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/java.png";
    game_state.card_picture[game_state.card_order[2] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/python.png";
    game_state.card_picture[game_state.card_order[3] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/kotlin.png";
    game_state.card_picture[game_state.card_order[4] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/prolog.png";
    game_state.card_picture[game_state.card_order[5] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/rust.png";
    game_state.card_picture[game_state.card_order[6] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/kotlin.png";
    game_state.card_picture[game_state.card_order[7] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/java.png";
    game_state.card_picture[game_state.card_order[8] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/rust.png";
    game_state.card_picture[game_state.card_order[9] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/haskell.png";
    game_state.card_picture[game_state.card_order[10] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/prolog.png";
    game_state.card_picture[game_state.card_order[11] as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/python.png";
    game_state.background_picture = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/background.png";
    game_state.exit_picture = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/exit.png";
    for i in 0 .. 12 {
        game_state.background_pictures[i as usize] = "/home/labus/Desktop/cao/Matching-Programming-Languages/resources/background.png";
    }

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
