use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static RADIUS: AtomicU32 = AtomicU32::new(0);
static OFFSET: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_offset(offset: f32) {
    OFFSET.store(offset.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Абсолютные координаты")]
async fn main() {
    set_radius(400.0);
    set_offset(200.0);

    loop {
        clear_background(BLACK);

        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));
        let offset = f32::from_bits(OFFSET.load(Ordering::Relaxed));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        draw_circle(center_x, center_y, radius, DARKBLUE);
        draw_circle(center_x + offset, center_y, 40.0, YELLOW);

        next_frame().await;
    }
}
