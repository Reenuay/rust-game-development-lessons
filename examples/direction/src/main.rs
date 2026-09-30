use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static LENGTH: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_length(length: f32) {
    LENGTH.store(length.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Направление")]
async fn main() {
    set_length(200.0);

    loop {
        clear_background(BLACK);

        let length = f32::from_bits(LENGTH.load(Ordering::Relaxed));

        // Неподвижный центр.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Расстояние от центра до курсора (как в уроке «Расстояние»).
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Нормированное направление — та же пара чисел, но длиной ровно 1.
        let direction_x = dx / distance;
        let direction_y = dy / distance;

        // Растягиваем направление до нужной длины.
        let end_x = center_x + direction_x * length;
        let end_y = center_y + direction_y * length;

        // Белая точка — неподвижный центр.
        draw_circle(center_x, center_y, 10.0, WHITE);
        // Жёлтый луч — направление на курсор, заданной длины.
        draw_line(center_x, center_y, end_x, end_y, 4.0, YELLOW);

        next_frame().await;
    }
}
