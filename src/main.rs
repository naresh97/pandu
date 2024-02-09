mod gui;
mod threading;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    //ui.window().set_fullscreen(true);
    ui.run().unwrap();
}
