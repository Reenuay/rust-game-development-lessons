use macroquad::prelude::*;

// Скорость круга в пикселях в секунду.
const SPEED: f32 = 300.0;

#[macroquad::main("Пауза")]
async fn main() {
    // Где сейчас круг по горизонтали.
    let mut x: f32 = 0.0;
    // Игра сейчас на паузе? Сначала нет.
    let mut paused = false;

    loop {
        clear_background(BLACK);

        // dt (delta time) — сколько секунд прошло между прошлым кадром и этим.
        let dt = get_frame_time();

        // Клавишу P только что нажали — включаем или выключаем паузу.
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }

        // Круг двигается, только пока игра не на паузе.
        if !paused {
            // Шаг за кадр — скорость, умноженная на dt.
            x += SPEED * dt;
            // Круг ушёл за правый край — возвращаем к левому.
            if x > screen_width() {
                x = 0.0;
            }
        }

        // Рисуем круг на каждом кадре, даже на паузе.
        draw_circle(x, screen_height() / 2.0, 40.0, YELLOW);

        // На паузе пишем об этом в углу.
        if paused {
            draw_text("Pause", 30.0, 110.0, 100.0, WHITE);
        }

        next_frame().await;
    }
}
