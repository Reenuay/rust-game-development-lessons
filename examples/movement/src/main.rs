use macroquad::prelude::*;

#[macroquad::main("Движение")]
async fn main() {
    // Начинаем с центра экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Стрелка вверх — двигаем вверх на 8 пикселей за кадр.
        if is_key_down(KeyCode::Up) {
            y -= 8.0;
        }
        // Стрелка вниз — двигаем вниз на 8 пикселей за кадр.
        if is_key_down(KeyCode::Down) {
            y += 8.0;
        }
        // Стрелка влево — двигаем влево на 8 пикселей за кадр.
        if is_key_down(KeyCode::Left) {
            x -= 8.0;
        }
        // Стрелка вправо — двигаем вправо на 8 пикселей за кадр.
        if is_key_down(KeyCode::Right) {
            x += 8.0;
        }

        draw_circle(x, y, 100.0, YELLOW);

        next_frame().await;
    }
}
