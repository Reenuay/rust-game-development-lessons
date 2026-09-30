use macroquad::prelude::*;

#[macroquad::main("Порядок отрисовки")]
async fn main() {
    // Кто рисуется вторым — и поэтому окажется сверху.
    let mut small_on_top = true;

    loop {
        clear_background(BLACK);

        // Клик — меняем местами порядок отрисовки.
        if is_mouse_button_pressed(MouseButton::Left) {
            small_on_top = !small_on_top;
        }

        // Центр экрана — оба круга рисуются в одной точке.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        if small_on_top {
            // Большой круг первым, маленький вторым — маленький виден поверх.
            draw_circle(center_x, center_y, 80.0, BLUE);
            draw_circle(center_x, center_y, 35.0, YELLOW);
        } else {
            // Маленький круг первым, большой вторым — маленький скрыт под ним.
            draw_circle(center_x, center_y, 35.0, YELLOW);
            draw_circle(center_x, center_y, 80.0, BLUE);
        }

        next_frame().await;
    }
}
