use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static DAMAGE: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_damage(damage: f32) {
    DAMAGE.store(damage.to_bits(), Ordering::Relaxed);
}

// Здоровье в начале и его максимум.
const MAX_HEALTH: f32 = 100.0;

// Левый верхний угол полоски, её высота и длина.
const BAR_X: f32 = 20.0;
const BAR_Y: f32 = 20.0;
const BAR_HEIGHT: f32 = 40.0;
const BAR_WIDTH: f32 = 400.0;

#[macroquad::main("Свой цвет")]
async fn main() {
    set_damage(10.0);

    // Текущее здоровье — сначала полное.
    let mut health = MAX_HEALTH;

    loop {
        clear_background(BLACK);

        let damage = f32::from_bits(DAMAGE.load(Ordering::Relaxed));

        // Кнопка мыши только что нажата.
        if is_mouse_button_pressed(MouseButton::Left) {
            if health <= 0.0 {
                // Здоровья уже нет — возвращаем полное.
                health = MAX_HEALTH;
            } else {
                // Отнимаем урон. max(0.0) не даёт здоровью уйти ниже нуля.
                health = (health - damage).max(0.0);
            }
        }

        // Доля здоровья от 0 до 1.
        let fraction = health / MAX_HEALTH;

        // Свой цвет: красного тем больше, чем меньше здоровья, а зелёного
        // тем больше, чем здоровья больше. Умножаем на 2: числа больше 1.0
        // функция считает за 1.0, поэтому посередине оба цвета на максимуме
        // и получается ярко-жёлтый.
        let color = Color::new(2.0 * (1.0 - fraction), 2.0 * fraction, 0.0, 1.0);

        // Серая подложка: вся полоска целиком.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH, BAR_HEIGHT, GRAY);
        // Цветная часть поверх неё: ширина — доля от всей полоски.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH * fraction, BAR_HEIGHT, color);

        next_frame().await;
    }
}
