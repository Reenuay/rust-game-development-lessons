use macroquad::prelude::*;

#[macroquad::main("Отрезок мышью")]
async fn main() {
    // Начальные точки — обе в центре, отрезок нулевой длины.
    let mut start_x = screen_width() / 2.0;
    let mut start_y = screen_height() / 2.0;
    let mut end_x = start_x;
    let mut end_y = start_y;

    loop {
        clear_background(BLACK);

        let (mouse_x, mouse_y) = mouse_position();

        // Истина только в тот кадр, когда кнопку только что нажали.
        if is_mouse_button_pressed(MouseButton::Left) {
            start_x = mouse_x;
            start_y = mouse_y;
        }

        // Истина, пока кнопка зажата.
        if is_mouse_button_down(MouseButton::Left) {
            end_x = mouse_x;
            end_y = mouse_y;
        }

        // is_mouse_button_released() тоже есть — истина в тот кадр,
        // когда кнопку отпустили. Отдельно она здесь не нужна: как
        // только is_mouse_button_down() станет false, end перестанет
        // обновляться сам собой.

        draw_line(start_x, start_y, end_x, end_y, 4.0, YELLOW);
        draw_circle(start_x, start_y, 10.0, WHITE);
        draw_circle(end_x, end_y, 10.0, WHITE);

        next_frame().await;
    }
}
