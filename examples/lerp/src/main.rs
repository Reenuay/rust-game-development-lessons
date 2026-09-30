use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static T: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_t(t: f32) {
    T.store(t.to_bits(), Ordering::Relaxed);
}

// Точка — как в уроке «Точка и прямоугольник». Ещё и Copy — start и end
// нужно и хранить между кадрами, и передавать в lerp одновременно.
#[derive(Clone, Copy)]
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

    // Начало и конец отрезка — по умолчанию на одной высоте.
    let mut start = Point { x: screen_width() * 0.2, y: screen_height() / 2.0 };
    let mut end = Point { x: screen_width() * 0.8, y: screen_height() / 2.0 };

    loop {
        clear_background(BLACK);

        // Читаем текущую долю пути.
        let t = f32::from_bits(T.load(Ordering::Relaxed));

        let (mouse_x, mouse_y) = mouse_position();

        // Отрезок можно перерисовать мышью — та же механика, что и в
        // уроке «Отрезок мышью»: нажатие ставит начало, зажатая кнопка
        // тянет конец за курсором.
        if is_mouse_button_pressed(MouseButton::Left) {
            start = Point { x: mouse_x, y: mouse_y };
        }
        if is_mouse_button_down(MouseButton::Left) {
            end = Point { x: mouse_x, y: mouse_y };
        }

        // Сам отрезок — как в уроке «Отрезок мышью».
        draw_line(start.x, start.y, end.x, end.y, 4.0, YELLOW);
        draw_circle(start.x, start.y, 10.0, WHITE);
        draw_circle(end.x, end.y, 10.0, WHITE);

        // Точка на отрезке, определяемая t.
        let point = lerp(start, end, t);
        draw_circle(point.x, point.y, 14.0, SKYBLUE);

        next_frame().await;
    }
}
