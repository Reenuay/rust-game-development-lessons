use macroquad::prelude::*;
use std::collections::HashMap;

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

// Кружок, которого держим на прицеле — его ID, а не номер в списке.
const TARGET_ID: u32 = 3;

#[macroquad::main("HashMap")]
async fn main() {
    // Ряд кружков: ключ — ID, значение — координаты. ID не нужно хранить
    // ещё и внутри значения — он и так ключ.
    let mut circles: HashMap<u32, (f32, f32)> = HashMap::new();
    for i in 0..COUNT {
        circles.insert(i as u32, (START_X + STEP * (i as f32), screen_height() / 2.0));
    }

    loop {
        clear_background(BLACK);

        // Клик только что случился — ищем, в какой кружок попали.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            // Сначала только читаем circles — удалять во время чтения нельзя.
            let mut found = false;
            let mut clicked_id: u32 = 0;

            for (&id, &(x, y)) in &circles {
                if distance(mouse_x, mouse_y, x, y) < RADIUS {
                    found = true;
                    clicked_id = id;
                }
            }

            // Теперь, когда чтение закончилось, можно убрать найденный кружок.
            if found {
                circles.remove(&clicked_id);
            }
        }

        // Рисуем каждый кружок, подписываем его ID и, если это и есть
        // TARGET_ID, тут же обводим его кольцом.
        for (&id, &(x, y)) in &circles {
            draw_circle(x, y, RADIUS, YELLOW);
            draw_text(&id.to_string(), x, y, 128.0, BLACK);

            if id == TARGET_ID {
                draw_circle_lines(x, y, RADIUS + 10.0, 4.0, WHITE);
            }
        }

        next_frame().await;
    }
}
