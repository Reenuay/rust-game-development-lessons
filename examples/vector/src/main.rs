use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static VECTOR_X: AtomicU32 = AtomicU32::new(0);
static VECTOR_Y: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_vector_x(vector_x: f32) {
    VECTOR_X.store(vector_x.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_vector_y(vector_y: f32) {
    VECTOR_Y.store(vector_y.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Точка плюс вектор")]
async fn main() {
    loop {
        clear_background(BLACK);

        let vector_x = f32::from_bits(VECTOR_X.load(Ordering::Relaxed));
        let vector_y = f32::from_bits(VECTOR_Y.load(Ordering::Relaxed));

        // Тот же вектор, но нарисованный от истинного (0, 0) — это и
        // есть смысл "точка = вектор из начала координат".
        draw_line(0.0, 0.0, vector_x, vector_y, 4.0, GRAY);
        draw_circle(vector_x, vector_y, 10.0, GRAY);

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Белая точка — сама позиция курсора, просто ориентир.
        draw_circle(mouse_x, mouse_y, 6.0, WHITE);

        // Тот же вектор, но прибавленный к курсору: точка + вектор.
        draw_line(mouse_x, mouse_y, mouse_x + vector_x, mouse_y + vector_y, 4.0, YELLOW);
        draw_circle(mouse_x + vector_x, mouse_y + vector_y, 10.0, YELLOW);

        next_frame().await;
    }
}
