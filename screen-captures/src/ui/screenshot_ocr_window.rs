use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use adw::gtk::{Box as GtkBox, ListBox, Orientation, SelectionMode};
use std::rc::Rc;
use std::cell::Cell;
use adw::glib::clone;
//use adw::gtk::glib;
use crate::ui::events::Events;

pub fn ui_test() -> Events {
    let application = Application::builder()
        .application_id("screenshot.ocr")
        .build();

    // Valor compartido entre closures
    // Events debe derivar Copy para poder usar Cell<Events>
    let result = Rc::new(Cell::new(Events::Cancelled));

    application.connect_activate(clone!(#[strong] result, move |app| {
        // Construir la ventana PRIMERO para poder capturarla en los closures
        let content = GtkBox::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Extract text?")
            .default_width(350)
            .content(&content)
            .build();

        // ActionRow "Yes"
        let row_yes = ActionRow::builder()
            .activatable(true)
            .title("Yes")
            .build();

        let result_yes = result.clone();
        row_yes.connect_activated(clone!(#[weak] window, move |_| {
            result_yes.set(Events::ScreenshotOcr);
            window.close();
        }));

        // ActionRow "No"
        let row_no = ActionRow::builder()
            .activatable(true)
            .title("No")
            .width_request(50)
            .build();

        let result_no = result.clone();
        row_no.connect_activated(clone!(#[weak] window, move |_| {
            result_no.set(Events::Screenshot);
            window.close();
        }));

        // ListBox
        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();

        list.append(&row_yes);
        list.append(&row_no);

        content.append(&list);

        window.present();
    }));

    application.run();
    result.get()
}