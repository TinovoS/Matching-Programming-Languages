use iced::executor;
use iced::{Application, Command, Element, Settings, Theme,Length};
use iced::widget::{Column,Row,Image};


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