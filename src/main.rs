use std::collections::HashMap;

use eframe::egui;
use eframe::NativeOptions;
use requester::request::Request;

fn main() -> Result<(), eframe::Error> {
    let req = Request::new(
        "GET".to_string(),
        "http://localhost:8088".to_string(),
        HashMap::new(),
        vec![],
    );

    println!("{:?}", req);
    draw()
}

fn draw() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([660.0, 800.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Requester".to_owned();
    let mut age = 42;

    eframe::run_simple_native("Send Request App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Requester");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));

            if ui.button("Increment").clicked() {
                age += 1;
            }

            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
