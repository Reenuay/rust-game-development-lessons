use macroquad::prelude::*;

#[macroquad::main("Ввод с клавиатуры")]
async fn main() {
    let size = 130.0;

    loop {
        clear_background(BLACK);

        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let offset_x = screen_width() * 0.16;
        let offset_y = screen_height() * 0.145;

        if is_key_down(KeyCode::Up) {
            let (ax, ay) = (cx, cy - offset_y);
            draw_triangle(
                vec2(ax, ay - size),
                vec2(ax - size, ay + size),
                vec2(ax + size, ay + size),
                RED,
            );
        }
        if is_key_down(KeyCode::Down) {
            let (ax, ay) = (cx, cy + offset_y);
            draw_triangle(
                vec2(ax, ay + size),
                vec2(ax - size, ay - size),
                vec2(ax + size, ay - size),
                YELLOW,
            );
        }
        if is_key_down(KeyCode::Left) {
            let (ax, ay) = (cx - offset_x, cy);
            draw_triangle(
                vec2(ax - size, ay),
                vec2(ax + size, ay - size),
                vec2(ax + size, ay + size),
                GREEN,
            );
        }
        if is_key_down(KeyCode::Right) {
            let (ax, ay) = (cx + offset_x, cy);
            draw_triangle(
                vec2(ax + size, ay),
                vec2(ax - size, ay - size),
                vec2(ax - size, ay + size),
                BLUE,
            );
        }

        next_frame().await;
    }
}
