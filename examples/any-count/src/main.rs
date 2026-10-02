use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_count(count: f32) {
    COUNT.store(count.to_bits(), Ordering::Relaxed);
}

// Откуда начинаем и на сколько пикселей сдвигаем каждый следующий кружок —
// то же самое, что и в уроке «Цикл for».
const START_X: f32 = 60.0;
const STEP: f32 = 80.0;

#[macroquad::main("Сколько угодно кружков")]
async fn main() {
    loop {
        clear_background(BLACK);

        let count = f32::from_bits(COUNT.load(Ordering::Relaxed)) as usize;

        // Сколько бы ни было кружков, цикл сам подстроится под count.
        for i in 0..count {
            let x = START_X + STEP * (i as f32);
            draw_circle(x, 60.0, 20.0, YELLOW);
        }

        next_frame().await;
    }
}
