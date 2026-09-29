use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static RADIUS: AtomicU32 = AtomicU32::new(0);
static PERCENT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_percent(percent: f32) {
    PERCENT.store(percent.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Относительные координаты")]
async fn main() {
    set_radius(400.0);
    set_percent(0.5);

    loop {
        clear_background(BLACK);

        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));
        let percent = f32::from_bits(PERCENT.load(Ordering::Relaxed));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Большой круг — по центру, радиусом R.
        draw_circle(center_x, center_y, radius, DARKBLUE);
        // Жёлтый кружок — смещён на долю P от радиуса R.
        draw_circle(center_x + radius * percent, center_y, 40.0, YELLOW);

        next_frame().await;
    }
}
