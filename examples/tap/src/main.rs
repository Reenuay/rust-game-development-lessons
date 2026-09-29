use macroquad::prelude::*;

#[macroquad::main("Рисуем в точке клика")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Кнопка зажата — рисуем круг прямо под курсором.
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            draw_circle(x, y, 40.0, YELLOW);
        }

        next_frame().await;
    }
}
