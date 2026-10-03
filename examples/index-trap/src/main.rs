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

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

// "Запомненный" кружок — просто его номер в списке, четвёртый по счёту.
const TARGET_INDEX: usize = 3;

#[macroquad::main("Ловушка индекса")]
async fn main() {
    // Начальный набор кружков — случайные позиции, радиусы и цвета.
    let mut circles: Vec<Circle> = Vec::new();
    for _ in 0..8 {
        circles.push(Circle {
            x: rand::gen_range(40.0, screen_width() - 40.0),
            y: rand::gen_range(40.0, screen_height() - 40.0),
            radius: rand::gen_range(50.0, 100.0),
            color: COLORS[rand::gen_range(0, COLORS.len())],
        });
    }

    loop {
        clear_background(BLACK);

        // Клик только что случился — убираем кружки под курсором.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            // Новый пустой список — специально для кружков, которые стоит оставить.
            let mut kept_circles: Vec<Circle> = Vec::new();

            // Забираем старый список circles в собственность.
            for circle in circles {
                let clicked_inside = distance(mouse_x, mouse_y, circle.x, circle.y) < circle.radius;

                if !clicked_inside {
                    kept_circles.push(circle);
                }
            }

            // На этот кадр circles становится тем, что мы только что собрали.
            circles = kept_circles;
        }

        // Рисуем каждый кружок и подписываем его индексом в списке.
        for i in 0..circles.len() {
            let circle = &circles[i];
            draw_circle(circle.x, circle.y, circle.radius, circle.color);
            draw_text(&i.to_string(), circle.x, circle.y, 128.0, BLACK);
        }

        // Кольцо вокруг "запомненного" кружка — только по номеру в списке.
        if TARGET_INDEX < circles.len() {
            let target = &circles[TARGET_INDEX];
            draw_circle_lines(target.x, target.y, target.radius + 10.0, 4.0, WHITE);
        }

        next_frame().await;
    }
}
