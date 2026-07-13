use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use adw::gtk::{Label, Box as GtkBox, Orientation, CssProvider};
use adw::gtk::gdk::Display;
use std::time::Duration;

pub fn clipboard_box() {
    let app = Application::builder()
        .application_id("dev.mephesto.capture-ocr")
        .build();

    app.connect_activate(|app| {
        show_temporal_window(app, "You can paste the extracted text from the clipboard", 2);
    });

    app.run();
}

/// Crea y muestra una ventana con un texto, sin botones,
/// que se cierra automaticamente despues de `seconds` segundos.
fn show_temporal_window(app: &Application, texto: &str, seconds: u64) {
    load_styles();

    let content = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .halign(adw::gtk::Align::Center)
        .valign(adw::gtk::Align::Center)
        .spacing(10)
        .css_classes(vec!["notificacion-caja"])
        .build();

    let label = Label::builder()
        .label(texto)
        .wrap(true)
        .justify(adw::gtk::Justification::Center)
        .build();

    content.append(&label);

    let window = ApplicationWindow::builder()
        .application(app)
        .content(&content)
        .default_width(320)
        .default_height(60)
        .resizable(false)
        .decorated(true)   
        .css_classes(vec!["notificacion-ventana"])
        .build();

    window.add_css_class("notificacion-ventana");

    window.present();

    let window_clone = window.clone();
    glib::timeout_add_local_once(Duration::from_secs(seconds), move || {
        window_clone.close();
    });
}

/// load css that gives GNOME style
fn load_styles() {
    let provider = CssProvider::new();
    provider.load_from_data(
        "
        window.notificacion-ventana {
            background-color: transparent;
        }

        box.notificacion-caja {
            background-color: rgba(30, 30, 30, 0.92);
            border-radius: 14px;
            padding: 14px 20px;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
        }

        box.notificacion-caja label {
            color: #ffffff;
            font-size: 14px;
        }
        ",
    );

    adw::gtk::style_context_add_provider_for_display(
        &Display::default().expect("No se pudo obtener el display por defecto"),
        &provider,
        adw::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
