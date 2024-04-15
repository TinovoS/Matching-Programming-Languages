use iced::executor;
use iced::{Application, Command, Element, Settings, Theme,Length};
use iced::widget::{Column,Row,Image};

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

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("Matching_Programming_Languages")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {

        let image_width = Length::Fill; // Each image takes 1/3 of the width
        let image_height = image_width; // Maintain aspect ratio

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
        Column::new()
            .spacing(10) // Add spacing between rows (optional)
            .push(
                Row::new()
                    .spacing(10) // Add spacing between images (optional)
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    )
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    )
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    ),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    )
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    )
                    .push(
                        Image::new("resources/image.jpeg")
                            .width(image_width)
                            .height(image_height),
                    ),
            )
            .into()
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
