use gtk::prelude::*;
use gtk::{Image, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Matching Programming Languages");
    window.set_default_size(1350, 900);

    let image = Image::new();
    image.set_from_file(Some("cat.png"));
    image.set_size_request(500, 600);
    

    window.add(&image);
    
    window.show_all();

    gtk::main();
}