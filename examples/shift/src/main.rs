use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static OFFSET_X: AtomicU32 = AtomicU32::new(0);
static OFFSET_Y: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_offset_x(offset_x: f32) {
    OFFSET_X.store(offset_x.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_offset_y(offset_y: f32) {
    OFFSET_Y.store(offset_y.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Смещение")]
async fn main() {
    loop {
        clear_background(BLACK);

        let offset_x = f32::from_bits(OFFSET_X.load(Ordering::Relaxed));
        let offset_y = f32::from_bits(OFFSET_Y.load(Ordering::Relaxed));

        let (mouse_x, mouse_y) = mouse_position();

        // Белая точка — сама позиция курсора, просто ориентир.
        draw_circle(mouse_x, mouse_y, 8.0, WHITE);

        // Жёлтый кружок — курсор плюс смещение.
        draw_circle(mouse_x + offset_x, mouse_y + offset_y, 40.0, YELLOW);

        next_frame().await;
    }
}
