use macroquad::prelude::*;

// Один круг: положение и радиус.
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle {
    // Пересекается ли этот круг с другим — сумма радиусов против расстояния.
    fn collides_with(&self, other: &Circle) -> bool {
        distance(self.x, self.y, other.x, other.y) < self.radius + other.radius
    }
}

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

#[macroquad::main("Столкновение кружков")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Неподвижный круг — всегда в центре экрана.
        let still = Circle {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            radius: 120.0,
        };

        // Круг, который ходит за мышью.
        let (mouse_x, mouse_y) = mouse_position();
        let moving = Circle { x: mouse_x, y: mouse_y, radius: 80.0 };

        // Цвет одинаковый у обоих — меняется, как только они пересеклись.
        let color = if moving.collides_with(&still) { RED } else { YELLOW };

        draw_circle(still.x, still.y, still.radius, color);
        draw_circle(moving.x, moving.y, moving.radius, color);

        next_frame().await;
    }
}
