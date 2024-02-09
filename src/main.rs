use std::time::Duration;

use chrono::{DateTime, Utc};
use slint::ComponentHandle;

mod gui;
mod threading;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(500),
        move || status_bar_loop(&ui_weak),
    );
    ui.run().unwrap();
}

fn status_bar_loop(ui_weak: &slint::Weak<MainWindow>) {
    if let Some(ui) = ui_weak.upgrade() {
        let time = std::time::SystemTime::now();
        let time: DateTime<Utc> = time.into();
        let time = time.format("%T, %e %b %Y").to_string();
        let data = StatusBarData { text: time.into() };
        ui.set_statusBarData(data);
    }
}
