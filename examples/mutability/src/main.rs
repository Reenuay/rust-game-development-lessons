use macroquad::prelude::*;

#[macroquad::main("Изменяемые переменные")]
async fn main() {
    // Начинаем с центра экрана.
    let mut x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        x += 1.0;

        draw_circle(x, y, 100.0, YELLOW);

        next_frame().await;
    }
}
