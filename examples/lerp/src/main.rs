use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static T: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_t(t: f32) {
    T.store(t.to_bits(), Ordering::Relaxed);
}

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Линейная интерполяция (lerp): точка на пути от from к to, где t — доля
// пройденного пути, от 0.0 (ещё в from) до 1.0 (уже в to).
fn lerp(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    }
}

#[macroquad::main("Линейная интерполяция")]
async fn main() {
    set_t(0.5);

    loop {
        clear_background(BLACK);

        // Читаем текущую долю пути.
        let t = f32::from_bits(T.load(Ordering::Relaxed));

        // Начало и конец отрезка — по умолчанию на одной высоте.
        let start = Point { x: screen_width() * 0.2, y: screen_height() / 2.0 };
        let end = Point { x: screen_width() * 0.8, y: screen_height() / 2.0 };

        // Сам отрезок.
        draw_line(start.x, start.y, end.x, end.y, 4.0, YELLOW);
        draw_circle(start.x, start.y, 10.0, WHITE);
        draw_circle(end.x, end.y, 10.0, WHITE);

        // Точка на отрезке, определяемая t.
        let point = lerp(start, end, t);
        draw_circle(point.x, point.y, 14.0, SKYBLUE);

        next_frame().await;
    }
}
