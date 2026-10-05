use macroquad::prelude::*;

// Фиксированный список цветов, из которого выбираем случайный.
const COLORS: [Color; 4] = [YELLOW, RED, GREEN, BLUE];

// Один кружок: положение, радиус и цвет вместе, под понятными именами.
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

#[macroquad::main("Свои структуры")]
async fn main() {
    // Без этого macroquad каждый запуск начинает с одних и тех же
    // «случайных» чисел. Время запуска даёт новый набор каждый раз.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Список уже нарисованных кружков — сначала пустой.
    let mut circles: Vec<Circle> = Vec::new();

    loop {
        clear_background(BLACK);

        // Клик — новый кружок со случайным радиусом и случайным цветом.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let radius = rand::gen_range(10.0, 60.0);
            let color = COLORS[rand::gen_range(0, COLORS.len())];
            circles.push(Circle { x, y, radius, color });
        }

        // Рисуем все уже сохранённые кружки.
        for circle in &circles {
            draw_circle(circle.x, circle.y, circle.radius, circle.color);
        }

        next_frame().await;
    }
}
