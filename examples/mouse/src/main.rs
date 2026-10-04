use macroquad::prelude::*;

#[macroquad::main("Клик мышью")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Левая кнопка мыши зажата — рисуем круг по центру экрана.
        if is_mouse_button_down(MouseButton::Left) {
            draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, YELLOW);
        }

        next_frame().await;
    }
}
