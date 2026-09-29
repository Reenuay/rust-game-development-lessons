use macroquad::prelude::*;

#[macroquad::main("Линия")]
async fn main() {
    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let (mouse_x, mouse_y) = mouse_position();

        draw_line(center_x, center_y, mouse_x, mouse_y, 4.0, YELLOW);

        next_frame().await;
    }
}
