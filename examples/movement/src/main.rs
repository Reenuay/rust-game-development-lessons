use macroquad::prelude::*;

// Скорость одна и та же для всех направлений, поэтому храним её
// в одном месте вместо того, чтобы писать число в четырёх местах.
const SPEED: f32 = 8.0;

#[macroquad::main("Движение")]
async fn main() {
    // Начинаем с центра экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Стрелка вверх — двигаем вверх.
        if is_key_down(KeyCode::Up) {
            y -= SPEED;
        }
        // Стрелка вниз — двигаем вниз.
        if is_key_down(KeyCode::Down) {
            y += SPEED;
        }
        // Стрелка влево — двигаем влево.
        if is_key_down(KeyCode::Left) {
            x -= SPEED;
        }
        // Стрелка вправо — двигаем вправо.
        if is_key_down(KeyCode::Right) {
            x += SPEED;
        }

        draw_circle(x, y, 75.0, YELLOW);

        next_frame().await;
    }
}
