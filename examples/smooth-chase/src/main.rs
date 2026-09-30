use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static FACTOR: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_factor(factor: f32) {
    FACTOR.store(factor.to_bits(), Ordering::Relaxed);
}

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Линейная интерполяция (lerp) — как в уроке «Линейная интерполяция».
fn lerp(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    }
}

#[macroquad::main("Плавная погоня")]
async fn main() {
    set_factor(0.08);

    // Кружок стартует в центре экрана. mut и вне loop — позиция должна
    // помнить прошлый кадр, а не сбрасываться каждый раз.
    let mut position = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };

    loop {
        clear_background(BLACK);

        let factor = f32::from_bits(FACTOR.load(Ordering::Relaxed));

        // Курсор — цель, к которой каждый кадр чуть-чуть приближаемся заново.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point { x: mouse_x, y: mouse_y };

        // Каждый кадр сдвигаемся на factor от текущего расстояния до
        // курсора: далеко от цели шаг большой, у цели — маленький.
        position = lerp(position, mouse, factor);

        draw_circle(position.x, position.y, 30.0, YELLOW);

        next_frame().await;
    }
}
