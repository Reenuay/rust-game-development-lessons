use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static BAR_WIDTH: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_bar_width(bar_width: f32) {
    BAR_WIDTH.store(bar_width.to_bits(), Ordering::Relaxed);
}

// Здоровье в начале и его максимум.
const MAX_HEALTH: f32 = 100.0;
// Сколько здоровья отнимает один клик.
const DAMAGE: f32 = 10.0;
// Выше какой доли здоровья полоска зелёная.
const GOOD_HEALTH: f32 = 0.6;
// Выше какой доли — жёлтая, а ниже — красная.
const LOW_HEALTH: f32 = 0.3;

// Левый верхний угол полоски и её высота.
const BAR_X: f32 = 20.0;
const BAR_Y: f32 = 20.0;
const BAR_HEIGHT: f32 = 40.0;

#[macroquad::main("else if")]
async fn main() {
    set_bar_width(400.0);

    // Текущее здоровье — сначала полное.
    let mut health = MAX_HEALTH;

    loop {
        clear_background(BLACK);

        let bar_width = f32::from_bits(BAR_WIDTH.load(Ordering::Relaxed));

        // Кнопка мыши только что нажата.
        if is_mouse_button_pressed(MouseButton::Left) {
            if health <= 0.0 {
                // Здоровья уже нет — возвращаем полное.
                health = MAX_HEALTH;
            } else {
                // Отнимаем урон. max(0.0) не даёт здоровью уйти ниже нуля.
                health = (health - DAMAGE).max(0.0);
            }
        }

        // Доля здоровья от 0 до 1.
        let fraction = health / MAX_HEALTH;

        // Цвет выбираем по очереди: много здоровья — зелёный, иначе
        // средне — жёлтый, иначе мало — красный.
        let color = if fraction > GOOD_HEALTH {
            GREEN
        } else if fraction > LOW_HEALTH {
            YELLOW
        } else {
            RED
        };

        // Серая подложка: вся полоска целиком.
        draw_rectangle(BAR_X, BAR_Y, bar_width, BAR_HEIGHT, GRAY);
        // Цветная часть поверх неё: ширина — доля от всей полоски.
        draw_rectangle(BAR_X, BAR_Y, bar_width * fraction, BAR_HEIGHT, color);

        next_frame().await;
    }
}
