use macroquad::prelude::*;

// Скорость движения — одна и та же на каждом кадре, как в уроке «Константы».
const SPEED: f32 = 8.0;

#[macroquad::main("WASD или стрелки")]
async fn main() {
    // Начинаем с центра экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Стрелка вверх или W — двигаем вверх на SPEED пикселей за кадр.
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y -= SPEED;
        }
        // Стрелка вниз или S — двигаем вниз на SPEED пикселей за кадр.
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y += SPEED;
        }
        // Стрелка влево или A — двигаем влево на SPEED пикселей за кадр.
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= SPEED;
        }
        // Стрелка вправо или D — двигаем вправо на SPEED пикселей за кадр.
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += SPEED;
        }

        draw_circle(x, y, 75.0, YELLOW);

        next_frame().await;
    }
}
