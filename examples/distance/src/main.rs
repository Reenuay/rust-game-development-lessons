use macroquad::prelude::*;

#[macroquad::main("Расстояние")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Центр экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Разница по x между курсором и центром.
        let dx = mouse_x - center_x;
        // Разница по y между курсором и центром.
        let dy = mouse_y - center_y;
        // Квадрат расстояния — сумма квадратов разниц по x и по y.
        let squared_distance = dx * dx + dy * dy;
        // Само расстояние — квадратный корень из squared_distance.
        let distance = squared_distance.sqrt();

        // draw_circle_lines — то же самое, что draw_circle, только
        // рисует контур, а не сплошную заливку.
        draw_circle_lines(center_x, center_y, distance, 3.0, YELLOW);

        // Белая точка — курсор, чтобы видно было, что контур касается
        // его ровно в этой точке.
        draw_circle(mouse_x, mouse_y, 8.0, WHITE);

        next_frame().await;
    }
}
