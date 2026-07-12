use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use adw::gtk::{Label, Box as GtkBox, Orientation};
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
    // Contenedor simple para centrar el texto
    let contenido = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .halign(adw::gtk::Align::Center)
        .valign(adw::gtk::Align::Center)
        .spacing(12)
        .build();

    let label = Label::builder()
        .label(texto)
        .wrap(true)
        .justify(adw::gtk::Justification::Center)
        .build();

    contenido.append(&label);

    let ventana = ApplicationWindow::builder()
        .application(app)
        .title("Aviso")
        .default_width(300)
        .default_height(150)
        .content(&contenido)
        .resizable(false)
        .build();

    ventana.present();

    // Programamos el cierre automático
    let ventana_clone = ventana.clone();
    glib::timeout_add_local_once(Duration::from_secs(seconds), move || {
        ventana_clone.close();
    });
}