extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.github.luoxiangyong.examples.gtk.demo"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("第一个GTK程序");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("点我啊！");
        button.connect_clicked(|_| {
            println!("哎呀，真的点了!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}


