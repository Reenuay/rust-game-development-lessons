use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static X_FRACTION: AtomicU32 = AtomicU32::new(0);
static Y_FRACTION: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_x_fraction(fraction: f32) {
    X_FRACTION.store(fraction.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_y_fraction(fraction: f32) {
    Y_FRACTION.store(fraction.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Доля от 0 до 1")]
async fn main() {
    set_x_fraction(0.5);
    set_y_fraction(0.5);

    loop {
        clear_background(BLACK);

        let x_fraction = f32::from_bits(X_FRACTION.load(Ordering::Relaxed));
        let y_fraction = f32::from_bits(Y_FRACTION.load(Ordering::Relaxed));

        let x = screen_width() * x_fraction;
        let y = screen_height() * y_fraction;

        draw_circle(x, y, 80.0, YELLOW);

        next_frame().await;
    }
}
