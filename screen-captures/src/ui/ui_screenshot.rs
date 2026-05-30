use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use adw::gtk::{Box, ListBox, Orientation, SelectionMode};

pub fn ui_test() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .title("Yes")
            .build();
        row.connect_activated(|_| {
            eprintln!("Extracting text from screenshot...");
        });

        let row_2 = ActionRow::builder()
            .activatable(true)
            .title("No")
            .width_request(50)
            .build();
        row_2.connect_activated(|_| {
            eprintln!("Taking screenshot...");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);
        list.append(&row_2);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Extract text?")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.present();
    });

    application.run();
}