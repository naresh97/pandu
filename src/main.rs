mod gui;
mod threading;

fn main() {
    let ui = MainWindow::new().unwrap();
    // ui.window().set_fullscreen(true);
    ui.run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window{
        Text {
            text: "hello!";
            color: green;
        }
    }
}
