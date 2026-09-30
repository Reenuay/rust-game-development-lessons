use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);
static ROTATE_SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_rotate_speed(rotate_speed: f32) {
    ROTATE_SPEED.store(rotate_speed.to_bits(), Ordering::Relaxed);
}

// Игрок — положение и угол поворота вместе, как в уроке «Свои структуры».
struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

impl Player {
    // Направление, куда сейчас смотрит игрок, — единичный вектор по
    // текущему углу (как в уроке «Синус и косинус»).
    fn direction(&self) -> (f32, f32) {
        (self.angle.cos(), self.angle.sin())
    }
}

#[macroquad::main("Поворот и движение")]
async fn main() {
    set_speed(4.0);
    set_rotate_speed(0.05);

    // Игрок стартует в центре экрана и смотрит вправо — угол 0.
    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        angle: 0.0,
    };

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));
        let rotate_speed = f32::from_bits(ROTATE_SPEED.load(Ordering::Relaxed));

        // Стрелка влево — поворот против часовой стрелки.
        if is_key_down(KeyCode::Left) {
            player.angle -= rotate_speed;
        }
        // Стрелка вправо — поворот по часовой стрелке.
        if is_key_down(KeyCode::Right) {
            player.angle += rotate_speed;
        }

        // Направление, куда сейчас смотрит игрок, — пересчитывается
        // заново на каждом кадре, сразу после поворота.
        let (direction_x, direction_y) = player.direction();

        // Стрелка вверх — шаг вперёд вдоль направления взгляда.
        if is_key_down(KeyCode::Up) {
            player.x += direction_x * speed;
            player.y += direction_y * speed;
        }
        // Стрелка вниз — шаг назад вдоль того же направления.
        if is_key_down(KeyCode::Down) {
            player.x -= direction_x * speed;
            player.y -= direction_y * speed;
        }

        // Тело игрока.
        draw_circle(player.x, player.y, 30.0, YELLOW);

        // Глазик — как у турели: точка плюс вектор, растянутый на 25
        // пикселей в направлении взгляда.
        let eye_x = player.x + direction_x * 25.0;
        let eye_y = player.y + direction_y * 25.0;
        draw_circle(eye_x, eye_y, 8.0, DARKBLUE);

        next_frame().await;
    }
}
