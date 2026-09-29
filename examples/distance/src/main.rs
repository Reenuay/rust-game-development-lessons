use macroquad::prelude::*;

#[macroquad::main("Расстояние")]
async fn main() {
    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let (mouse_x, mouse_y) = mouse_position();

        // Расстояние по x и по y отдельно, потом в одну формулу.
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // draw_circle_lines — то же самое, что draw_circle, только
        // рисует контур, а не сплошную заливку.
        draw_circle_lines(center_x, center_y, distance, 3.0, YELLOW);

        // Белая точка — курсор, чтобы видно было, что контур касается
        // его ровно в этой точке.
        draw_circle(mouse_x, mouse_y, 8.0, WHITE);

        next_frame().await;
    }
}
