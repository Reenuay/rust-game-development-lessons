use macroquad::prelude::*;

#[macroquad::main("Ввод с клавиатуры")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Стрелка вверх зажата — рисуем круг по центру экрана.
        if is_key_down(KeyCode::Up) {
            draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, YELLOW);
        }

        next_frame().await;
    }
}
