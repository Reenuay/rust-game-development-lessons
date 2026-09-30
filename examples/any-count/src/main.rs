use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_count(count: f32) {
    COUNT.store(count.to_bits(), Ordering::Relaxed);
}

// Ширина, на которой кружки равномерно распределены.
const WIDTH: f32 = 600.0;

#[macroquad::main("Сколько угодно кружков")]
async fn main() {
    loop {
        clear_background(BLACK);

        let count = f32::from_bits(COUNT.load(Ordering::Relaxed)) as usize;

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Считаем шаг, только если кружков больше одного — иначе делили
        // бы на ноль (как в уроке «Деление на ноль»).
        let step = if count > 1 { WIDTH / (count - 1) as f32 } else { 0.0 };
        // Средний индекс — точка отсчёта для смещения каждого кружка.
        let mid = (count - 1) as f32 / 2.0;

        // Сколько бы ни было кружков, цикл сам подстроится под count.
        for i in 0..count {
            let x = center_x + (i as f32 - mid) * step;
            draw_circle(x, center_y, 20.0, YELLOW);
        }

        next_frame().await;
    }
}
