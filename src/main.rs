use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main(){
    let app = Application::builder()
    .application_id(".com.matching_programming_languages")
    .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
    .title("Matching Programming Languages")
    .application(app)
    .build();

    window.show();
}