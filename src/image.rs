use extern crate egui
use egui::ImageData;
use std::path::Path;


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