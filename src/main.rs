use std::cell::RefCell;
use std::clone;
use std::rc::Rc;

use glib_macros::clone;
use gtk::{gio::ResourceLookupFlags, prelude::*};
use uuid::Uuid;

fn main() {
    let application = gtk::Application::new(Some("com.donabe8898.dev"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("UUID gen"));
    window.set_default_size(500, 500);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Start)
        .spacing(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // 子へ
    window.set_child(Some(&vbox));
    let uuid_txt = gtk::Text::new();
    let button = gtk::Button::with_label("Gen UUID");

    let clone_uuid_txt = uuid_txt.clone();
    button.connect_clicked(move |_| {
        let id = Uuid::new_v4();
        let str_id = id.to_string();
        clone_uuid_txt.set_text(&str_id);
    });
    vbox.append(&uuid_txt);
    vbox.append(&button);

    window.show();
}

// fn build_button() -> gtk::Button {}
