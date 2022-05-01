extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::Application;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("Layout.glade");

    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.object("applicationwindow1").unwrap();

    window.set_application(Some(app));

    // Inputs:
    let message_input: gtk::Entry = builder.object("message").unwrap();

    // Submit button:
    let button: gtk::Button = builder.object("btn_generate").unwrap();

    // Outputs:
    let message_output: gtk::Label = builder.object("message_output").unwrap();

    let image_output: gtk::Image = builder.object("image_output").unwrap();

    let image_output_clone = image_output.clone(); // we make a clone of image to move its ownership to the button "connect_clicked" closure...
                                                   // if we don't clone it, we could not then hide the image

    button.connect_clicked(move |_| {
        message_output.set_text(&format!("{}\n   /\n  /", message_input.text().as_str()));
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application = Application::builder()
        .application_id("com.shayan.catsay")
        .build();

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run();
}
