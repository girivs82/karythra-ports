#![no_std]
#![no_main]

use karythra::wm;
use karythra::io;
use karythra::ui::Ui;

#[karythra::main]
async fn main() {
    io::print("guitest: starting KaryaUI\n");

    let win = match wm::Window::create(400, 300, "KaryaUI Test").await {
        Ok(w) => w,
        Err(_) => { io::print("guitest: window create failed\n"); return; }
    };

    io::print("guitest: window created\n");

    let mut counter: u32 = 0;
    let mut frame: u32 = 0;

    loop {
        frame += 1;
        let events = win.poll_events().await;
        let mut ui = Ui::begin(&win, &events).await;

        ui.label("KaryaUI Demo").await;
        ui.separator().await;
        ui.label("Scene-graph immediate-mode UI").await;
        ui.space(4.0).await;

        ui.row_start();
        if ui.button("Click me").await {
            counter += 1;
        }
        if ui.button("Reset").await {
            counter = 0;
        }
        ui.row_end();

        ui.space(4.0).await;

        let mut buf = [0u8; 20];
        let n = karythra::fmt::decimal(counter as u64, &mut buf);
        if let Ok(s) = core::str::from_utf8(&buf[..n]) {
            ui.label(s).await;
        }

        ui.end(&win).await;

        karythra::time::sleep(16).await;
    }
}
