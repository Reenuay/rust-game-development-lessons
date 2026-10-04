use macroquad::prelude::*;
use std::collections::HashMap;

// Радиус один и тот же у всех кружков — фиксированный, как в уроке «Константы».
const RADIUS: f32 = 60.0;

#[macroquad::main("Автоинкремент")]
async fn main() {
    // Кружки, появляющиеся по клику: ключ — ID, значение — координаты.
    let mut circles: HashMap<u32, (f32, f32)> = HashMap::new();
    // Следующий свободный ID — растёт на единицу с каждым новым кружком.
    let mut next_id: u32 = 0;

    loop {
        clear_background(BLACK);

        // Кнопка мыши только что нажата — добавляем новый кружок.
        if is_mouse_button_pressed(MouseButton::Left) {
            // Текущее значение счётчика становится ID нового кружка.
            circles.insert(next_id, mouse_position());
            // Следующий кружок получит число на единицу больше.
            next_id += 1;
        }

        // Рисуем каждый кружок и подписываем его ID.
        for (&id, &(x, y)) in &circles {
            draw_circle(x, y, RADIUS, YELLOW);
            draw_text(&id.to_string(), x, y, 64.0, BLACK);
        }

        next_frame().await;
    }
}
