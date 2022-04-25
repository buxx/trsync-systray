use tray_item::TrayItem;
use std::sync::mpsc;

enum Message {
    Quit,
}

pub fn run_tray() -> Result<(), String> {
    let mut tray = match TrayItem::new("Tracim", "my-icon-name") {
        Ok(tray_) => tray_,
        Err(error) => return Err(format!("Unable to create tray item : '{}'", error)),
    };

    match tray.add_menu_item("Configurer", || {
        println!("Hello!");
    }) {
        Err(error) => return Err(format!("Unable to add menu item : '{:?}'", error)),
        _ => {}
    };

    let (tx, rx) = mpsc::channel();

    tray.add_menu_item("Quitter", move || {
        tx.send(Message::Quit)
            .expect("This channel must not been closed");
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => break,
            _ => {}
        }
    }

    Ok(())
}
