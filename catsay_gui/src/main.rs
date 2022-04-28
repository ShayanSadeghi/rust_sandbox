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
    window.show_all();
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
