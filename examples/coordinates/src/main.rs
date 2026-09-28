use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static POS_X: AtomicU32 = AtomicU32::new(0);
static POS_Y: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_x(x: f32) {
    POS_X.store(x.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_y(y: f32) {
    POS_Y.store(y.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Координаты")]
async fn main() {
    set_x(screen_width() / 2.0);
    set_y(screen_height() / 2.0);

    loop {
        clear_background(BLACK);

        let x = f32::from_bits(POS_X.load(Ordering::Relaxed));
        let y = f32::from_bits(POS_Y.load(Ordering::Relaxed));
        draw_circle(x, y, 40.0, YELLOW);

        next_frame().await;
    }
}
