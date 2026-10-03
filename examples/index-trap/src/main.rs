use macroquad::prelude::*;

// Радиус один и тот же у всех кружков — фиксированный, как в уроке «Константы».
const RADIUS: f32 = 84.0;

// Сколько кружков рисуем и как их расставить в ряд — те же START_X и STEP,
// что и в уроке «Цикл for».
const COUNT: usize = 10;
const START_X: f32 = 100.0;
const STEP: f32 = 228.0;

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
    // Начальный набор кружков — ровный ряд, одного радиуса и цвета.
    let mut circles: Vec<(f32, f32)> = Vec::new();
    for i in 0..COUNT {
        circles.push((START_X + STEP * (i as f32), screen_height() / 2.0));
    }

    loop {
        clear_background(BLACK);

        // Клик только что случился — убираем кружки под курсором.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            // Новый пустой список — специально для кружков, которые стоит оставить.
            let mut kept_circles: Vec<(f32, f32)> = Vec::new();

            // Забираем старый список circles в собственность.
            for (x, y) in circles {
                let clicked_inside = distance(mouse_x, mouse_y, x, y) < RADIUS;

                if !clicked_inside {
                    kept_circles.push((x, y));
                }
            }

            // На этот кадр circles становится тем, что мы только что собрали.
            circles = kept_circles;
        }

        // Рисуем каждый кружок и подписываем его индексом в списке.
        for i in 0..circles.len() {
            let (x, y) = circles[i];
            draw_circle(x, y, RADIUS, YELLOW);
            draw_text(&i.to_string(), x, y, 128.0, BLACK);
        }

        // Кольцо вокруг "запомненного" кружка — только по номеру в списке.
        if TARGET_INDEX < circles.len() {
            let (x, y) = circles[TARGET_INDEX];
            draw_circle_lines(x, y, RADIUS + 10.0, 4.0, WHITE);
        }

        next_frame().await;
    }
}
