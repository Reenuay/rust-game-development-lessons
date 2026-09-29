use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

// Своя функция: по двум точкам считает направление от первой ко второй.
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    // Вектор от начальной точки к конечной.
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    // Длина этого вектора — расстояние между точками.
    let distance = (dx * dx + dy * dy).sqrt();

    // Точки не совпадают — можно посчитать настоящее направление.
    if distance > 0.0 {
        // Делим на длину — получаем направление длиной ровно 1.
        (dx / distance, dy / distance)
    } else {
        // Точки совпали — направления нет, берём по умолчанию вправо.
        (1.0, 0.0)
    }
}

#[macroquad::main("Свои функции")]
async fn main() {
    // Кружок стартует в центре экрана. mut и вне loop — позиция
    // должна помнить прошлый кадр, а не сбрасываться каждый раз.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от кружка к курсору — вся математика внутри функции.
        let (direction_x, direction_y) = direction(x, y, mouse_x, mouse_y);

        // Точка + вектор: шаг фиксированной длины в эту сторону.
        x += direction_x * speed;
        y += direction_y * speed;

        draw_circle(x, y, 30.0, YELLOW);

        next_frame().await;
    }
}
