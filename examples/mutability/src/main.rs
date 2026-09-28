use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static POS_X: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_x(x: f32) {
    POS_X.store(x.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn get_x() -> f32 {
    f32::from_bits(POS_X.load(Ordering::Relaxed))
}

#[macroquad::main("Изменяемые переменные")]
async fn main() {
    // Начинаем с центра экрана.
    set_x(screen_width() / 2.0);
    let y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        let mut x = get_x();
        x += 1.0;
        set_x(x);

        draw_circle(x, y, 100.0, YELLOW);

        next_frame().await;
    }
}
