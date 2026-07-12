use arboard::{Clipboard, SetExtLinux};

pub fn write_clipboard(text: String) {
    let mut clip = Clipboard::new().unwrap();

    clip.set().wait().text(text).unwrap();
    //clip.set_text(text).unwrap();
}
