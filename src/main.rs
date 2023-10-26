use std::cell::Cell;
use std::rc::Rc;
use gtk4 as gtk;
use gtk::prelude::*;
use glib::clone;
use gtk::{glib, Application, ApplicationWindow, Button};

fn init_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();


    let counter = Rc::new(Cell::new(0));
    //let counter_refb = counter_refa.clone();

    button_increase.connect_clicked(clone!(@strong counter => move |button| {
        counter.set(counter.get() + 1);
        button.set_label(&format!("Contador: {}", counter.get()))
    }));
    button_decrease.connect_clicked(move |button| {
        counter.set(counter.get() - 1);
        button.set_label(&format!("Contador: {}", counter.get()))
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("HexEditor")
        .default_width(1024)
        .default_height(768)
        .child(&button_increase)
        .child(&button_decrease)
        .build();
    
    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.antonhyonhi.HexEditor")
        .build();
    app.connect_activate(init_ui);
    app.run()
}