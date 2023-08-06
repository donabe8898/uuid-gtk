use gtk::prelude::*;
use uuid::Uuid;

fn main() {
    let application: gtk::Application =
        gtk::Application::new(Some("com.donabe8898.dev"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("UUID gen"));
    window.set_default_size(500, 500);

    let vbox: gtk::Box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .spacing(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // 子へ
    window.set_child(Some(&vbox));

    let uuid_txt: gtk::Text = gtk::Text::new();
    let button: gtk::Button = gtk::Button::with_label("Gen UUID");

    let clone_uuid_txt: gtk::Text = uuid_txt.clone();
    button.connect_clicked(move |_| {
        let id: Uuid = Uuid::new_v4();
        let str_id: String = id.to_string();
        clone_uuid_txt.set_text(&str_id);
    });

    vbox.append(&uuid_txt);
    vbox.append(&button);

    window.show();
}

// fn build_button() -> gtk::Button {}
