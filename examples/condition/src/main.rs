use macroquad::prelude::*;

#[macroquad::main("Иначе")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        if is_mouse_button_down(MouseButton::Left) {
            draw_circle(center_x, center_y - 150.0, 80.0, YELLOW);
        } else {
            draw_circle(center_x, center_y - 150.0, 80.0, RED);
        }

        draw_circle(center_x, center_y + 150.0, 80.0, BLUE);

        next_frame().await;
    }
}
