extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Box, Button, Image, Label, Orientation};

fn main() {
    let application = Application::builder()
        .application_id("com.shayan.catsay")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK App")
            .default_width(350)
            .default_height(70)
            .build();

        let layout_box = Box::new(Orientation::Vertical, 0);

        let label = Label::new(Some("Meow!\n        \\\n         \\\n"));

        layout_box.add(&label);

        let cat_image = Image::from_file("./images/cat.png");

        layout_box.add(&cat_image);
        window.add(&layout_box);
        window.show_all()
    });
    application.run();
}
