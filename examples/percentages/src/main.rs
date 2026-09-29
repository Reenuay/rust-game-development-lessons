use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static X_PERCENT: AtomicU32 = AtomicU32::new(0);
static Y_PERCENT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_x_percent(percent: f32) {
    X_PERCENT.store(percent.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_y_percent(percent: f32) {
    Y_PERCENT.store(percent.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Проценты")]
async fn main() {
    set_x_percent(50.0);
    set_y_percent(50.0);

    loop {
        clear_background(BLACK);

        let x_percent = f32::from_bits(X_PERCENT.load(Ordering::Relaxed));
        let y_percent = f32::from_bits(Y_PERCENT.load(Ordering::Relaxed));

        // Процент от ширины экрана.
        let x = screen_width() * x_percent / 100.0;
        // Процент от высоты экрана.
        let y = screen_height() * y_percent / 100.0;

        draw_circle(x, y, 80.0, YELLOW);

        next_frame().await;
    }
}
