

use eframe::egui::{self, include_image, TextureHandle};
use egui::ImageData;
use std::path::Path;
use std::{thread, time};
use egui::{RichText, FontId, Color32};


const NUM_OF_CARDS: usize = 24;



#[derive(Default)]
pub struct GameState {
    picked_globaly: [bool; NUM_OF_CARDS ],
    picked_localy: [bool; NUM_OF_CARDS ],
    card_order: [i8; NUM_OF_CARDS ],
    card_picture: [&'static str; NUM_OF_CARDS],
    picked_count: u8,
    pick_number: u8,
    correct_pairs: u8,
    card: i8,
    reset: bool,
    reset_count: i64,
    background_pictures: [&'static str; NUM_OF_CARDS],
    background_picture: &'static str,
    exit_picture: &'static str,
    play:bool,
    normal:bool,
    show_popup: bool,
    image_size: i64,
    level: i8,
    number_of_cards: usize,
    victory: bool,
    victory_picture: &'static str,
    logo_picture: &'static str,
    start:bool,
}


impl GameState {
    /// Called once before the first frame.
    pub fn new() -> Self {
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
            reset: false, //maybe will use it
            reset_count: 0, // this counts frames when 2 cards are clicked so they reset after certain amout of frames
            background_pictures: [""; NUM_OF_CARDS], // foreach card background
            background_picture: "", // card picture for background
            exit_picture: "", // picture for imagebutton exit
            play: false, // if play is clicked then turn to true and game is on
            normal: true,
            show_popup: false,
            image_size: 0,
            level: 1,
            number_of_cards: 12,
            victory: false,
            victory_picture: "resources/victory.png",
            logo_picture: "resources/logo(1).png",
            start: true
        };

        // Initialize the state
        // Return the initialized GameState
        game_state
    }
}
impl eframe::App for GameState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        let my_frame = egui::containers::Frame {
            inner_margin: egui::Margin { left: 10., right: 10., top: 10., bottom: 10. },
            outer_margin: egui::Margin { left: 10., right: 10., top: 10., bottom: 10. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { offset: [1.0, 2.0].into(), blur: 0.0, spread:0.0, color: Color32::YELLOW },
            fill: egui::Color32::from_rgba_premultiplied(201,194,188,255),
            stroke: egui::Stroke::new(2.0, Color32::GOLD),
        };
        //lets initialize game status
        //------------------------------------
        //------------------------------------
        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
            if self.victory{

                let available_size = ctx.screen_rect().size();
                egui::Grid::new("victory")
                    .num_columns(1)
                    .min_col_width(available_size.x)
                    //.min_row_height(min_row_height)
                    .max_col_width(available_size.x)

                    .show(ui, |mut row| {

                        const GRAY: egui::Color32 = egui::Color32::from_rgb(38, 38, 38);

                        let victory = row.ctx().load_texture(
                            "my-image",
                            get_image(&self.victory_picture, 0, 0, 100, 100),
                            Default::default(),
                        );

                        let victory_button = egui::ImageButton::new(&victory);
                        if row.add(victory_button).clicked() {
                            full_reset(self);
                            self.play = false;
                            self.victory=false;
                        }
                    });
            }
            else if self.play {
                if ((ctx.screen_rect().size().x - 6.0 * 210.0 > 0.0) && ((ctx.screen_rect().size().y - 140.0) -(self.level as f32 + 1.0) * 210.0 > 0.0)){
                    self.image_size = 200;
                    load_image2(self);

                }
                else if((ctx.screen_rect().size().x - 6.0 * 158.0 > 0.0) && ((ctx.screen_rect().size().y - 140.0) -(self.level as f32 + 1.0) * 158.0 > 0.0)){
                    self.image_size = 150;
                    load_image2(self);

                }
                else if((ctx.screen_rect().size().x - 6.0 * 108.0 > 0.0) && ((ctx.screen_rect().size().y - 140.0) -(self.level as f32 + 1.0) * 108.0 > 0.0)){
                    self.image_size = 100;
                    load_image2(self);

                }
                else {
                    self.image_size = 75;
                    load_image2(self);

                }
                if self.picked_count == 2{
                    self.reset_count += 1;  // this imiates  count like when it counts to 1000 and
                }
                let mut num_rows:usize =0;
                let num_columns = 6;
                if(self.normal) {
                    num_rows = 1 + self.level as usize;

                }
                else {
                    num_rows = 3;
                }
                let available_size = ctx.screen_rect().size();
                let min_col_width = (available_size.x*0.98) / num_columns as f32;
                let min_row_height = (available_size.y*0.99 - 180.0)  / num_rows as f32;


                //here we represent labels
                egui::Grid::new("header")
                    .num_columns(4)
                    //.min_row_height(min_row_height)
                    .min_col_width(available_size.x*0.99  / 4.0)
                    .show(ui, |mut row| {

                        const GRAY: egui::Color32 = egui::Color32::from_rgb(38, 38, 38);

                        let exit_img = row.ctx().load_texture(
                            "my-image",
                            get_image(&self.exit_picture, 0, 0, 100, 100),
                            Default::default(),
                        );

                        let exit_button = egui::ImageButton::new(&exit_img);
                        if row.add(exit_button).clicked() {
                            full_reset(self);
                            self.start = true;
                            self.play = false;
                        }


                        /*let framed_text_label = egui::Frame::none()
                            //.inner_margin(egui::Margin::symmetric(available_size.x*0.99  / 6.0,0.0))
                            //.fill(GRAY)
                            .show(row, |ui| {
                                ui.label(RichText::new(""));
                            });
*/
                        let another_text_label = "Another Text Here";
                        let framed_another_text_label = egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(available_size.x*0.99  / 6.0,0.0))
                            .fill(GRAY)
                            .show(row, |ui| {
                                ui.label(RichText::new("Level: ".to_owned() + &*(self.level).to_string()).font(FontId::proportional(40.0)));
                            });

                    });


                for n in 0..num_rows  {
                    egui::Grid::new(n)
                        .num_columns(num_columns)
                        .min_col_width(min_col_width)
                        .min_row_height(min_row_height)
                        .max_col_width(min_col_width)
                        .show(ui, |mut row| {
                            for m in 0..num_columns  {
                                if(self.picked_count == self.number_of_cards as u8 / 2){

                                }
                                let mut background_img = row.ctx().load_texture(
                                    "my-image",
                                    get_image(&self.background_pictures[n * num_columns + m], 0, 0, 100, 100),
                                    Default::default(),
                                );

                                if self.picked_globaly[n * num_columns + m] {
                                    row.add(egui::Image::new(&background_img));
                                }

                                else if row.add(egui::ImageButton::new(&background_img)).clicked()  {
                                    if self.picked_count == 2{
                                        reset_background_cards(self);
                                    }
                                    self.picked_localy[n * num_columns + m] = true;
                                    self.picked_globaly[n * num_columns + m] = true;
                                    self.background_pictures[n * num_columns + m] = self.card_picture[n * num_columns + m];

                                    self.picked_count +=1;

                                }


                                if self.picked_count == 2 && self.reset_count==6{
                                    reset_background_cards(self);
                                }

                            }
                        });
                }
            }
            else {
                if self.start {
                    if ctx.screen_rect().size().x > 900.0 {
                        self.image_size = 200;
                        self.logo_picture = "resources/logo(2).png";
                    }
                    else{
                        self.image_size = 100;
                        self.logo_picture = "resources/logo(1).png";

                    }
                    ui.vertical_centered_justified(|ui| {
                        let available_size = ctx.screen_rect().size();
                        let min_col_width = available_size.x*6.0/10.0   as f32;
                        let min_row_height = (available_size.y/10.0) as f32;
                        let inter_row_spacing = min_row_height/2.0;
                        let center = available_size.x/5.0;
                        let mut center_image = available_size.x/2.0;
                        if(self.image_size != 200) {
                            center_image = center_image - 192.0;
                        }
                        else {
                            center_image = center_image - 384.0;
                        }
                        ui.horizontal(|ui| {
                            ui.add_space(center_image);

                            let mut logo_image = ui.ctx().load_texture(
                                "my-image",
                                get_image(&self.logo_picture, 0, 0, 100, 100),
                                Default::default(),
                            );

                            ui.add(egui::Image::new(&logo_image));
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if !self.play {
                                if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("Normal").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                    self.show_popup = true;
                                    self.start = false;
                                    self.normal = true;
                                    self.level = 3;
                                    self.number_of_cards = 24;
                                    // Initialize the state
                                }
                            }
                            else {
                                ui.label("Game is playing...");
                            }
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if !self.play {
                                if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("Extreme").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                    self.show_popup = true;
                                    self.normal = false;
                                    self.start = false;
                                    self.number_of_cards = 18;
                                    self.level = 3;

                                }
                            }
                            else {
                                ui.label("Game is playing...");
                            }
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("Quit").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                std::process::exit(0);
                            }
                        });

                    });
                }
                if self.show_popup {
                    ui.vertical_centered_justified(|ui| {
                        let available_size = ctx.screen_rect().size();
                        let min_col_width = available_size.x*6.0/10.0   as f32;
                        let min_row_height = (available_size.y/10.0) as f32;
                        let inter_row_spacing = min_row_height/2.0;
                        let center = available_size.x/5.0;
                        let mut center_image = available_size.x/2.0;
                        if ctx.screen_rect().size().x > 900.0 {
                            self.image_size = 200;
                            self.logo_picture = "resources/logo(2).png";
                        }
                        else{
                            self.image_size = 100;
                            self.logo_picture = "resources/logo(1).png";

                        }
                        if(self.image_size == 100) {
                            center_image = center_image - 192.0;
                        }
                        else{
                            center_image = center_image - 384.0;
                        }
                        ui.horizontal(|ui| {
                            ui.add_space(center_image);

                            let mut logo_image = ui.ctx().load_texture(
                                "my-image",
                                get_image(&self.logo_picture, 0, 0, 100, 100),
                                Default::default(),
                            );

                            ui.add(egui::Image::new(&logo_image));
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if !self.play {
                                if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("100px").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                    self.play = true;
                                    self.show_popup = false;
                                    randomize_cards(self);

                                }
                            }
                            else {
                                ui.label("Game is playing...");
                            }
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if !self.play {
                                if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("200px").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                    self.play = true;
                                    self.show_popup = false;
                                    randomize_cards(self);

                                }
                            }
                            else {
                                ui.label("Game is playing...");
                            }
                        });
                        ui.add_space(inter_row_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(center);

                            if ui.add_sized([min_col_width, min_row_height],egui::Button::new(RichText::new("Back").size(100.0*ctx.screen_rect().size().x  /1900 as f32).strong())).clicked() {
                                self.show_popup = false;
                                self.start = true;
                            }
                        });

                    });
                }
            } // Add a lot of widgets here.
            });
        });
    }
}

fn compare_cards(game_state: &mut GameState) -> () {
    let mut buffer: [&'static str; 2] = [""; 2];
    let mut k: usize = 0;
    for i in 0..game_state.number_of_cards {
        if game_state.picked_localy[i] {
            buffer[k] = game_state.background_pictures[i];
            k +=1;
        }
    }


    if buffer[0] == buffer[1]{
        game_state.correct_pairs +=1;
        if game_state.correct_pairs == (game_state.number_of_cards as u8) /2 && game_state.level < 3{
            next_level(game_state);
            game_state.level += 1;
            game_state.number_of_cards+=6;
            randomize_cards(game_state);
        }
        else if game_state.correct_pairs == (game_state.number_of_cards as u8) /2 && game_state.level == 3{
            game_state.victory = true;
            game_state.play = false;
        }
        else if(!game_state.normal){
            randomize_cards(game_state);
        }
    }else {
        for i in 0..game_state.number_of_cards {
            if game_state.picked_localy[i]{
                game_state.picked_globaly[i] = false;
            }
        }
    }
}

fn reset_background_cards(game_state: &mut GameState) {
    compare_cards(game_state);

    // Reset picked count
    game_state.picked_count = 0;
    game_state.reset_count = 0;
    for i in 0..game_state.number_of_cards {
        game_state.picked_localy[i] =false;
    }

    for i in 0..game_state.number_of_cards {
        if game_state.picked_globaly[i] == false {
            if game_state.image_size == 200 {
                game_state.background_pictures[i as usize] =
                    "resources/200x200/background.png";
            } else if game_state.image_size == 100{
                game_state.background_pictures[i as usize] =
                    "resources/100x100/background.png";
            }
            else if game_state.image_size == 150 {
                game_state.background_pictures[i as usize] =
                    "resources/150x150/background.png";
            } else if game_state.image_size == 75{
                game_state.background_pictures[i as usize] =
                    "resources/75x75/background.png";
            }
        }
    }
}
fn full_reset(mut game_state: &mut GameState) -> () {
    game_state.pick_number = 0;
    game_state.correct_pairs = 0;
    game_state.card = -1;
    game_state.picked_localy = [false; NUM_OF_CARDS];
    game_state.picked_globaly= [false; NUM_OF_CARDS];
    game_state.card_order= [-1; NUM_OF_CARDS]; //card order used in randomize
    game_state.picked_count= 0; // count how many cards are picked
    game_state.reset= false; //maybe will use it
    game_state.reset_count= 0; // this counts frames when 2 cards are clicked so they reset after certain amout of frames
    game_state.play= false; // if play is clicked then turn to true and game is on
    game_state.normal= true;
    game_state.show_popup= false;
    game_state.level= 1;
    game_state.victory = false;
}
fn next_level(mut game_state: &mut GameState) -> () {
    game_state.pick_number = 0;
    game_state.correct_pairs = 0;
    game_state.card = -1;
    game_state.picked_localy = [false; NUM_OF_CARDS];
    game_state.picked_globaly= [false; NUM_OF_CARDS];
    game_state.card_order= [-1; NUM_OF_CARDS]; //card order used in randomize
    game_state.picked_count= 0; // count how many cards are picked
    game_state.reset_count= 0; // this counts frames when 2 cards are clicked so they reset after certain amout of frames
}
fn randomize_cards(mut game_state: &mut GameState) -> () {
    for n in 0..game_state.number_of_cards {
        if (game_state.card_order[n] != -1 && !game_state.picked_globaly[game_state.card_order[n] as usize]) {
            game_state.card_order[n] = -1;
        }
    }
    for n in 0..game_state.number_of_cards {
        if (game_state.card_order[n] != -1) {continue;}
        let mut x = (rand::random::<i8>()).rem_euclid(game_state.number_of_cards as i8);
        while game_state.card_order.contains(&x) {
            x = (rand::random::<i8>()).rem_euclid(game_state.number_of_cards as i8);
        }
        game_state.card_order[n] = x;
        game_state.picked_localy[n] = false;
    }
    load_image(game_state);
    let num_columns = 6;
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
fn load_image2(mut game_state: &mut GameState) -> () {

    if game_state.image_size == 200{
        {game_state.card_picture[game_state.card_order[0] as usize] = "resources/200x200/haskell.png";}
        {game_state.card_picture[game_state.card_order[1] as usize] = "resources/200x200/haskell.png";}
        {game_state.card_picture[game_state.card_order[2] as usize] = "resources/200x200/kotlin.png";}
        {game_state.card_picture[game_state.card_order[3] as usize] = "resources/200x200/kotlin.png";}
        {game_state.card_picture[game_state.card_order[4] as usize] = "resources/200x200/prolog.png";}
        {game_state.card_picture[game_state.card_order[5] as usize] = "resources/200x200/prolog.png";}
        {game_state.card_picture[game_state.card_order[6] as usize] = "resources/200x200/java.png";}
        {game_state.card_picture[game_state.card_order[7] as usize] = "resources/200x200/java.png";}
        {game_state.card_picture[game_state.card_order[8] as usize] = "resources/200x200/rust.png";}
        {game_state.card_picture[game_state.card_order[9] as usize] = "resources/200x200/rust.png";}
        {game_state.card_picture[game_state.card_order[10] as usize] = "resources/200x200/python.png";}
        {game_state.card_picture[game_state.card_order[11] as usize] = "resources/200x200/python.png";}
        if(game_state.number_of_cards >=18) {
            { game_state.card_picture[game_state.card_order[12] as usize] = "resources/200x200/scala.png"; }
            { game_state.card_picture[game_state.card_order[13] as usize] = "resources/200x200/scala.png"; }
            { game_state.card_picture[game_state.card_order[14] as usize] = "resources/200x200/c++.png"; }
            { game_state.card_picture[game_state.card_order[15] as usize] = "resources/200x200/c++.png"; }
            { game_state.card_picture[game_state.card_order[16] as usize] = "resources/200x200/lisp.png"; }
            { game_state.card_picture[game_state.card_order[17] as usize] = "resources/200x200/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            { game_state.card_picture[game_state.card_order[18] as usize] = "resources/200x200/fortran.png"; }
            { game_state.card_picture[game_state.card_order[19] as usize] = "resources/200x200/fortran.png"; }
            { game_state.card_picture[game_state.card_order[20] as usize] = "resources/200x200/ruby.png"; }
            { game_state.card_picture[game_state.card_order[21] as usize] = "resources/200x200/ruby.png"; }
            { game_state.card_picture[game_state.card_order[22] as usize] = "resources/200x200/go.png"; }
            { game_state.card_picture[game_state.card_order[23] as usize] = "resources/200x200/go.png"; }
        }
        game_state.background_picture = "resources/200x200/background.png";
        game_state.exit_picture = "resources/logo.png";
        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/200x200/background.png";
            }
        }
    } else if game_state.image_size == 150{
        {game_state.card_picture[game_state.card_order[0] as usize] = "resources/150x150/haskell.png";}
        {game_state.card_picture[game_state.card_order[1] as usize] = "resources/150x150/haskell.png";}
        {game_state.card_picture[game_state.card_order[2] as usize] = "resources/150x150/kotlin.png";}
        {game_state.card_picture[game_state.card_order[3] as usize] = "resources/150x150/kotlin.png";}
        {game_state.card_picture[game_state.card_order[4] as usize] = "resources/150x150/prolog.png";}
        {game_state.card_picture[game_state.card_order[5] as usize] = "resources/150x150/prolog.png";}
        {game_state.card_picture[game_state.card_order[6] as usize] = "resources/150x150/java.png";}
        {game_state.card_picture[game_state.card_order[7] as usize] = "resources/150x150/java.png";}
        {game_state.card_picture[game_state.card_order[8] as usize] = "resources/150x150/rust.png";}
        {game_state.card_picture[game_state.card_order[9] as usize] = "resources/150x150/rust.png";}
        {game_state.card_picture[game_state.card_order[10] as usize] = "resources/150x150/python.png";}
        {game_state.card_picture[game_state.card_order[11] as usize] = "resources/150x150/python.png";}
        if(game_state.number_of_cards >=18) {
            { game_state.card_picture[game_state.card_order[12] as usize] = "resources/150x150/scala.png"; }
            { game_state.card_picture[game_state.card_order[13] as usize] = "resources/150x150/scala.png"; }
            { game_state.card_picture[game_state.card_order[14] as usize] = "resources/150x150/c++.png"; }
            { game_state.card_picture[game_state.card_order[15] as usize] = "resources/150x150/c++.png"; }
            { game_state.card_picture[game_state.card_order[16] as usize] = "resources/150x150/lisp.png"; }
            { game_state.card_picture[game_state.card_order[17] as usize] = "resources/150x150/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            { game_state.card_picture[game_state.card_order[18] as usize] = "resources/150x150/fortran.png"; }
            { game_state.card_picture[game_state.card_order[19] as usize] = "resources/150x150/fortran.png"; }
            { game_state.card_picture[game_state.card_order[20] as usize] = "resources/150x150/ruby.png"; }
            { game_state.card_picture[game_state.card_order[21] as usize] = "resources/150x150/ruby.png"; }
            { game_state.card_picture[game_state.card_order[22] as usize] = "resources/150x150/go.png"; }
            { game_state.card_picture[game_state.card_order[23] as usize] = "resources/150x150/go.png"; }
        }
        game_state.background_picture = "resources/150x150/background.png";
        game_state.exit_picture = "resources/logo.png";
        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/150x150/background.png";
            }
        }
    }
    else if game_state.image_size == 100{
         {game_state.card_picture[game_state.card_order[0] as usize] = "resources/100x100/haskell.png";}
       {game_state.card_picture[game_state.card_order[1] as usize] = "resources/100x100/haskell.png";}
         {game_state.card_picture[game_state.card_order[2] as usize] = "resources/100x100/kotlin.png";}
        {game_state.card_picture[game_state.card_order[3] as usize] = "resources/100x100/kotlin.png";}
        {game_state.card_picture[game_state.card_order[4] as usize] = "resources/100x100/prolog.png";}
        {game_state.card_picture[game_state.card_order[5] as usize] = "resources/100x100/prolog.png";}
        {game_state.card_picture[game_state.card_order[6] as usize] = "resources/100x100/java.png";}
        {game_state.card_picture[game_state.card_order[7] as usize] = "resources/100x100/java.png";}
        {game_state.card_picture[game_state.card_order[8] as usize] = "resources/100x100/rust.png";}
        {game_state.card_picture[game_state.card_order[9] as usize] = "resources/100x100/rust.png";}
        {game_state.card_picture[game_state.card_order[10] as usize] = "resources/100x100/python.png";}
        {game_state.card_picture[game_state.card_order[11] as usize] = "resources/100x100/python.png";}
        if(game_state.number_of_cards >=18) {
            { game_state.card_picture[game_state.card_order[12] as usize] = "resources/100x100/scala.png"; }
            { game_state.card_picture[game_state.card_order[13] as usize] = "resources/100x100/scala.png"; }
            { game_state.card_picture[game_state.card_order[14] as usize] = "resources/100x100/c++.png"; }
            { game_state.card_picture[game_state.card_order[15] as usize] = "resources/100x100/c++.png"; }
            { game_state.card_picture[game_state.card_order[16] as usize] = "resources/100x100/lisp.png"; }
            { game_state.card_picture[game_state.card_order[17] as usize] = "resources/100x100/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            { game_state.card_picture[game_state.card_order[18] as usize] = "resources/100x100/fortran.png"; }
            { game_state.card_picture[game_state.card_order[19] as usize] = "resources/100x100/fortran.png"; }
            { game_state.card_picture[game_state.card_order[20] as usize] = "resources/100x100/ruby.png"; }
            { game_state.card_picture[game_state.card_order[21] as usize] = "resources/100x100/ruby.png"; }
            { game_state.card_picture[game_state.card_order[22] as usize] = "resources/100x100/go.png"; }
            { game_state.card_picture[game_state.card_order[23] as usize] = "resources/100x100/go.png"; }
        }
        game_state.background_picture = "resources/100x100/background.png";
        game_state.exit_picture = "resources/logo.png";

        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/100x100/background.png";
            }
        }
    }
    else if game_state.image_size == 75{
        {game_state.card_picture[game_state.card_order[0] as usize] = "resources/75x75/haskell.png";}
        {game_state.card_picture[game_state.card_order[1] as usize] = "resources/75x75/haskell.png";}
        {game_state.card_picture[game_state.card_order[2] as usize] = "resources/75x75/kotlin.png";}
        {game_state.card_picture[game_state.card_order[3] as usize] = "resources/75x75/kotlin.png";}
        {game_state.card_picture[game_state.card_order[4] as usize] = "resources/75x75/prolog.png";}
        {game_state.card_picture[game_state.card_order[5] as usize] = "resources/75x75/prolog.png";}
        {game_state.card_picture[game_state.card_order[6] as usize] = "resources/75x75/java.png";}
        {game_state.card_picture[game_state.card_order[7] as usize] = "resources/75x75/java.png";}
        {game_state.card_picture[game_state.card_order[8] as usize] = "resources/75x75/rust.png";}
        {game_state.card_picture[game_state.card_order[9] as usize] = "resources/75x75/rust.png";}
        {game_state.card_picture[game_state.card_order[10] as usize] = "resources/75x75/python.png";}
        {game_state.card_picture[game_state.card_order[11] as usize] = "resources/75x75/python.png";}
        if(game_state.number_of_cards >=18) {
            { game_state.card_picture[game_state.card_order[12] as usize] = "resources/75x75/scala.png"; }
            { game_state.card_picture[game_state.card_order[13] as usize] = "resources/75x75/scala.png"; }
            { game_state.card_picture[game_state.card_order[14] as usize] = "resources/75x75/c++.png"; }
            { game_state.card_picture[game_state.card_order[15] as usize] = "resources/75x75/c++.png"; }
            { game_state.card_picture[game_state.card_order[16] as usize] = "resources/75x75/lisp.png"; }
            { game_state.card_picture[game_state.card_order[17] as usize] = "resources/75x75/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            { game_state.card_picture[game_state.card_order[18] as usize] = "resources/75x75/fortran.png"; }
            { game_state.card_picture[game_state.card_order[19] as usize] = "resources/75x75/fortran.png"; }
            { game_state.card_picture[game_state.card_order[20] as usize] = "resources/75x75/ruby.png"; }
            { game_state.card_picture[game_state.card_order[21] as usize] = "resources/75x75/ruby.png"; }
            { game_state.card_picture[game_state.card_order[22] as usize] = "resources/75x75/go.png"; }
            { game_state.card_picture[game_state.card_order[23] as usize] = "resources/75x75/go.png"; }
        }
        game_state.background_picture = "resources/75x75/background.png";
        game_state.exit_picture = "resources/logo.png";

        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/75x75/background.png";
            }
        }
    }
        for i in 0..game_state.number_of_cards {
            if (game_state.picked_localy[i] || game_state.picked_globaly[i]) {
                game_state.background_pictures[i] = game_state.card_picture[i];
            }

    }
}
fn load_image(mut game_state: &mut GameState) -> () {

    if game_state.image_size == 200{
        if(!game_state.picked_globaly[game_state.card_order[0] as usize]) {game_state.card_picture[game_state.card_order[0] as usize] = "resources/200x200/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[1] as usize]) {game_state.card_picture[game_state.card_order[1] as usize] = "resources/200x200/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[2] as usize]) {game_state.card_picture[game_state.card_order[2] as usize] = "resources/200x200/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[3] as usize]) {game_state.card_picture[game_state.card_order[3] as usize] = "resources/200x200/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[4] as usize]) {game_state.card_picture[game_state.card_order[4] as usize] = "resources/200x200/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[5] as usize]) {game_state.card_picture[game_state.card_order[5] as usize] = "resources/200x200/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[6] as usize]) {game_state.card_picture[game_state.card_order[6] as usize] = "resources/200x200/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[7] as usize]) {game_state.card_picture[game_state.card_order[7] as usize] = "resources/200x200/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[8] as usize]) {game_state.card_picture[game_state.card_order[8] as usize] = "resources/200x200/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[9] as usize]) {game_state.card_picture[game_state.card_order[9] as usize] = "resources/200x200/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[10] as usize]) {game_state.card_picture[game_state.card_order[10] as usize] = "resources/200x200/python.png";}
        if(!game_state.picked_globaly[game_state.card_order[11] as usize]) {game_state.card_picture[game_state.card_order[11] as usize] = "resources/200x200/python.png";}
        if(game_state.number_of_cards >=18) {
            if (!game_state.picked_globaly[game_state.card_order[12] as usize]) { game_state.card_picture[game_state.card_order[12] as usize] = "resources/200x200/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[13] as usize]) { game_state.card_picture[game_state.card_order[13] as usize] = "resources/200x200/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[14] as usize]) { game_state.card_picture[game_state.card_order[14] as usize] = "resources/200x200/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[15] as usize]) { game_state.card_picture[game_state.card_order[15] as usize] = "resources/200x200/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[16] as usize]) { game_state.card_picture[game_state.card_order[16] as usize] = "resources/200x200/lisp.png"; }
            if (!game_state.picked_globaly[game_state.card_order[17] as usize]) { game_state.card_picture[game_state.card_order[17] as usize] = "resources/200x200/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            if (!game_state.picked_globaly[game_state.card_order[18] as usize]) { game_state.card_picture[game_state.card_order[18] as usize] = "resources/200x200/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[19] as usize]) { game_state.card_picture[game_state.card_order[19] as usize] = "resources/200x200/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[20] as usize]) { game_state.card_picture[game_state.card_order[20] as usize] = "resources/200x200/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[21] as usize]) { game_state.card_picture[game_state.card_order[21] as usize] = "resources/200x200/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[22] as usize]) { game_state.card_picture[game_state.card_order[22] as usize] = "resources/200x200/go.png"; }
            if (!game_state.picked_globaly[game_state.card_order[23] as usize]) { game_state.card_picture[game_state.card_order[23] as usize] = "resources/200x200/go.png"; }
        }
        game_state.background_picture = "resources/200x200/background.png";
        game_state.exit_picture = "resources/logo.png";
        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/200x200/background.png";
            }
        }
    } else if game_state.image_size == 150{
        if(!game_state.picked_globaly[game_state.card_order[0] as usize]) {game_state.card_picture[game_state.card_order[0] as usize] = "resources/150x150/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[1] as usize]) {game_state.card_picture[game_state.card_order[1] as usize] = "resources/150x150/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[2] as usize]) {game_state.card_picture[game_state.card_order[2] as usize] = "resources/150x150/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[3] as usize]) {game_state.card_picture[game_state.card_order[3] as usize] = "resources/150x150/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[4] as usize]) {game_state.card_picture[game_state.card_order[4] as usize] = "resources/150x150/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[5] as usize]) {game_state.card_picture[game_state.card_order[5] as usize] = "resources/150x150/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[6] as usize]) {game_state.card_picture[game_state.card_order[6] as usize] = "resources/150x150/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[7] as usize]) {game_state.card_picture[game_state.card_order[7] as usize] = "resources/150x150/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[8] as usize]) {game_state.card_picture[game_state.card_order[8] as usize] = "resources/150x150/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[9] as usize]) {game_state.card_picture[game_state.card_order[9] as usize] = "resources/150x150/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[10] as usize]) {game_state.card_picture[game_state.card_order[10] as usize] = "resources/150x150/python.png";}
        if(!game_state.picked_globaly[game_state.card_order[11] as usize]) {game_state.card_picture[game_state.card_order[11] as usize] = "resources/150x150/python.png";}
        if(game_state.number_of_cards >=18) {
            if (!game_state.picked_globaly[game_state.card_order[12] as usize]) { game_state.card_picture[game_state.card_order[12] as usize] = "resources/150x150/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[13] as usize]) { game_state.card_picture[game_state.card_order[13] as usize] = "resources/150x150/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[14] as usize]) { game_state.card_picture[game_state.card_order[14] as usize] = "resources/150x150/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[15] as usize]) { game_state.card_picture[game_state.card_order[15] as usize] = "resources/150x150/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[16] as usize]) { game_state.card_picture[game_state.card_order[16] as usize] = "resources/150x150/lisp.png"; }
            if (!game_state.picked_globaly[game_state.card_order[17] as usize]) { game_state.card_picture[game_state.card_order[17] as usize] = "resources/150x150/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            if (!game_state.picked_globaly[game_state.card_order[18] as usize]) { game_state.card_picture[game_state.card_order[18] as usize] = "resources/150x150/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[19] as usize]) { game_state.card_picture[game_state.card_order[19] as usize] = "resources/150x150/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[20] as usize]) { game_state.card_picture[game_state.card_order[20] as usize] = "resources/150x150/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[21] as usize]) { game_state.card_picture[game_state.card_order[21] as usize] = "resources/150x150/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[22] as usize]) { game_state.card_picture[game_state.card_order[22] as usize] = "resources/150x150/go.png"; }
            if (!game_state.picked_globaly[game_state.card_order[23] as usize]) { game_state.card_picture[game_state.card_order[23] as usize] = "resources/150x150/go.png"; }
        }
        game_state.background_picture = "resources/150x150/background.png";
        game_state.exit_picture = "resources/logo.png";
        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/150x150/background.png";
            }
        }
    }
    else if game_state.image_size == 100{
        if(!game_state.picked_globaly[game_state.card_order[0] as usize]) {game_state.card_picture[game_state.card_order[0] as usize] = "resources/100x100/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[1] as usize]) {game_state.card_picture[game_state.card_order[1] as usize] = "resources/100x100/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[2] as usize]) {game_state.card_picture[game_state.card_order[2] as usize] = "resources/100x100/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[3] as usize]) {game_state.card_picture[game_state.card_order[3] as usize] = "resources/100x100/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[4] as usize]) {game_state.card_picture[game_state.card_order[4] as usize] = "resources/100x100/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[5] as usize]) {game_state.card_picture[game_state.card_order[5] as usize] = "resources/100x100/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[6] as usize]) {game_state.card_picture[game_state.card_order[6] as usize] = "resources/100x100/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[7] as usize]) {game_state.card_picture[game_state.card_order[7] as usize] = "resources/100x100/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[8] as usize]) {game_state.card_picture[game_state.card_order[8] as usize] = "resources/100x100/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[9] as usize]) {game_state.card_picture[game_state.card_order[9] as usize] = "resources/100x100/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[10] as usize]) {game_state.card_picture[game_state.card_order[10] as usize] = "resources/100x100/python.png";}
        if(!game_state.picked_globaly[game_state.card_order[11] as usize]) {game_state.card_picture[game_state.card_order[11] as usize] = "resources/100x100/python.png";}
        if(game_state.number_of_cards >=18) {
            if (!game_state.picked_globaly[game_state.card_order[12] as usize]) { game_state.card_picture[game_state.card_order[12] as usize] = "resources/100x100/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[13] as usize]) { game_state.card_picture[game_state.card_order[13] as usize] = "resources/100x100/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[14] as usize]) { game_state.card_picture[game_state.card_order[14] as usize] = "resources/100x100/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[15] as usize]) { game_state.card_picture[game_state.card_order[15] as usize] = "resources/100x100/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[16] as usize]) { game_state.card_picture[game_state.card_order[16] as usize] = "resources/100x100/lisp.png"; }
            if (!game_state.picked_globaly[game_state.card_order[17] as usize]) { game_state.card_picture[game_state.card_order[17] as usize] = "resources/100x100/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            if (!game_state.picked_globaly[game_state.card_order[18] as usize]) { game_state.card_picture[game_state.card_order[18] as usize] = "resources/100x100/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[19] as usize]) { game_state.card_picture[game_state.card_order[19] as usize] = "resources/100x100/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[20] as usize]) { game_state.card_picture[game_state.card_order[20] as usize] = "resources/100x100/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[21] as usize]) { game_state.card_picture[game_state.card_order[21] as usize] = "resources/100x100/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[22] as usize]) { game_state.card_picture[game_state.card_order[22] as usize] = "resources/100x100/go.png"; }
            if (!game_state.picked_globaly[game_state.card_order[23] as usize]) { game_state.card_picture[game_state.card_order[23] as usize] = "resources/100x100/go.png"; }
        }
        game_state.background_picture = "resources/100x100/background.png";
        game_state.exit_picture = "resources/logo.png";

        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/100x100/background.png";
            }
        }
    }
    else if game_state.image_size == 75{
        if(!game_state.picked_globaly[game_state.card_order[0] as usize]) {game_state.card_picture[game_state.card_order[0] as usize] = "resources/75x75/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[1] as usize]) {game_state.card_picture[game_state.card_order[1] as usize] = "resources/75x75/haskell.png";}
        if(!game_state.picked_globaly[game_state.card_order[2] as usize]) {game_state.card_picture[game_state.card_order[2] as usize] = "resources/75x75/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[3] as usize]) {game_state.card_picture[game_state.card_order[3] as usize] = "resources/75x75/kotlin.png";}
        if(!game_state.picked_globaly[game_state.card_order[4] as usize]) {game_state.card_picture[game_state.card_order[4] as usize] = "resources/75x75/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[5] as usize]) {game_state.card_picture[game_state.card_order[5] as usize] = "resources/75x75/prolog.png";}
        if(!game_state.picked_globaly[game_state.card_order[6] as usize]) {game_state.card_picture[game_state.card_order[6] as usize] = "resources/75x75/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[7] as usize]) {game_state.card_picture[game_state.card_order[7] as usize] = "resources/75x75/java.png";}
        if(!game_state.picked_globaly[game_state.card_order[8] as usize]) {game_state.card_picture[game_state.card_order[8] as usize] = "resources/75x75/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[9] as usize]) {game_state.card_picture[game_state.card_order[9] as usize] = "resources/75x75/rust.png";}
        if(!game_state.picked_globaly[game_state.card_order[10] as usize]) {game_state.card_picture[game_state.card_order[10] as usize] = "resources/75x75/python.png";}
        if(!game_state.picked_globaly[game_state.card_order[11] as usize]) {game_state.card_picture[game_state.card_order[11] as usize] = "resources/75x75/python.png";}
        if(game_state.number_of_cards >=18) {
            if (!game_state.picked_globaly[game_state.card_order[12] as usize]) { game_state.card_picture[game_state.card_order[12] as usize] = "resources/75x75/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[13] as usize]) { game_state.card_picture[game_state.card_order[13] as usize] = "resources/75x75/scala.png"; }
            if (!game_state.picked_globaly[game_state.card_order[14] as usize]) { game_state.card_picture[game_state.card_order[14] as usize] = "resources/75x75/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[15] as usize]) { game_state.card_picture[game_state.card_order[15] as usize] = "resources/75x75/c++.png"; }
            if (!game_state.picked_globaly[game_state.card_order[16] as usize]) { game_state.card_picture[game_state.card_order[16] as usize] = "resources/75x75/lisp.png"; }
            if (!game_state.picked_globaly[game_state.card_order[17] as usize]) { game_state.card_picture[game_state.card_order[17] as usize] = "resources/75x75/lisp.png"; }
        }
        if(game_state.number_of_cards ==24) {
            if (!game_state.picked_globaly[game_state.card_order[18] as usize]) { game_state.card_picture[game_state.card_order[18] as usize] = "resources/75x75/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[19] as usize]) { game_state.card_picture[game_state.card_order[19] as usize] = "resources/75x75/fortran.png"; }
            if (!game_state.picked_globaly[game_state.card_order[20] as usize]) { game_state.card_picture[game_state.card_order[20] as usize] = "resources/75x75/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[21] as usize]) { game_state.card_picture[game_state.card_order[21] as usize] = "resources/75x75/ruby.png"; }
            if (!game_state.picked_globaly[game_state.card_order[22] as usize]) { game_state.card_picture[game_state.card_order[22] as usize] = "resources/75x75/go.png"; }
            if (!game_state.picked_globaly[game_state.card_order[23] as usize]) { game_state.card_picture[game_state.card_order[23] as usize] = "resources/75x75/go.png"; }
        }
        game_state.background_picture = "resources/75x75/background.png";
        game_state.exit_picture = "resources/logo.png";

        for i in 0 .. game_state.number_of_cards {
            if(!game_state.picked_globaly[i]) {
                game_state.background_pictures[i as usize] = "resources/75x75/background.png";
            }
        }
    }
    for i in 0..game_state.number_of_cards {
        if (game_state.picked_localy[i] || game_state.picked_globaly[i]) {
            game_state.background_pictures[i] = game_state.card_picture[i];
        }

    }
}