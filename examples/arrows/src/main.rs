use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static HEAD_LENGTH: AtomicU32 = AtomicU32::new(0);
static HEAD_WIDTH: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_head_length(head_length: f32) {
    HEAD_LENGTH.store(head_length.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_head_width(head_width: f32) {
    HEAD_WIDTH.store(head_width.to_bits(), Ordering::Relaxed);
}

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Рисует стрелку из from в to — линию с треугольным наконечником.
fn draw_arrow(from: Point, to: Point, head_length: f32, head_width: f32, color: Color) {
    // Направление и длина — как в «Направлении».
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let length = (dx * dx + dy * dy).sqrt();

    // Нулевой вектор не на что направить — как в «Делении на ноль».
    if length == 0.0 {
        return;
    }

    let direction_x = dx / length;
    let direction_y = dy / length;

    // Шаг назад от кончика вдоль направления.
    let back_x = to.x - direction_x * head_length;
    let back_y = to.y - direction_y * head_length;

    // Перпендикуляр к направлению — поворот на 90°, как в «Повороте на 90°».
    let perp_x = -direction_y;
    let perp_y = direction_x;

    // Два «крыла» наконечника — по разные стороны от back. Каждая
    // координата — своя переменная, чтобы было видно, как она
    // считается.
    let wing1_x = back_x + perp_x * head_width;
    let wing1_y = back_y + perp_y * head_width;
    let wing2_x = back_x - perp_x * head_width;
    let wing2_y = back_y - perp_y * head_width;

    // Тело стрелки — до back, а не до самого кончика, чтобы не вылезать
    // из-под наконечника.
    draw_line(from.x, from.y, back_x, back_y, 4.0, color);

    // draw_triangle() в macroquad хочет точки в виде Vec2, а не просто
    // x и y по отдельности — vec2() здесь просто упаковывает уже
    // готовые координаты.
    draw_triangle(
        vec2(to.x, to.y),
        vec2(wing1_x, wing1_y),
        vec2(wing2_x, wing2_y),
        color,
    );
}

#[macroquad::main("Стрелки")]
async fn main() {
    set_head_length(24.0);
    set_head_width(12.0);

    loop {
        clear_background(BLACK);

        let head_length = f32::from_bits(HEAD_LENGTH.load(Ordering::Relaxed));
        let head_width = f32::from_bits(HEAD_WIDTH.load(Ordering::Relaxed));

        // Стрелка летит из центра экрана в курсор мыши.
        let center = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };
        let (mouse_x, mouse_y) = mouse_position();
        let cursor = Point { x: mouse_x, y: mouse_y };

        draw_arrow(center, cursor, head_length, head_width, YELLOW);

        next_frame().await;
    }
}
