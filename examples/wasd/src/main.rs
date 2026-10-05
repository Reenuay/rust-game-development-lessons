use macroquad::prelude::*;

#[macroquad::main("WASD или стрелки")]
async fn main() {
    // Начинаем с центра экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Стрелка вверх или W — двигаем вверх на 8 пикселей за кадр.
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y -= 8.0;
        }
        // Стрелка вниз или S — двигаем вниз на 8 пикселей за кадр.
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y += 8.0;
        }
        // Стрелка влево или A — двигаем влево на 8 пикселей за кадр.
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= 8.0;
        }
        // Стрелка вправо или D — двигаем вправо на 8 пикселей за кадр.
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += 8.0;
        }

        draw_circle(x, y, 75.0, YELLOW);

        next_frame().await;
    }
}
