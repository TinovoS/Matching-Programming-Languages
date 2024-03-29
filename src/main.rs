use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("My Window");
    window.set_default_size(350, 70);

    window.show_all();

    gtk::main();
}