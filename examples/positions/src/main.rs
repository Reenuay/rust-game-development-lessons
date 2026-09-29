use macroquad::prelude::*;

#[macroquad::main("Позиционирование")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Ширина и высота экрана.
        let w = screen_width();
        let h = screen_height();

        // Центр: ровно половина ширины и высоты.
        draw_circle(w / 2.0, h / 2.0, 80.0, YELLOW);

        // Остальные четыре точки — деление на 3 вместо 2, как в сетке
        // "правила третей" из фотографии: экран мысленно делится на
        // девять равных прямоугольников, и объекты ставят на
        // пересечения линий, а не строго по центру.
        draw_circle(w / 3.0, h / 3.0, 80.0, RED);
        draw_circle(w / 3.0 * 2.0, h / 3.0, 80.0, GREEN);
        draw_circle(w / 3.0, h / 3.0 * 2.0, 80.0, BLUE);
        draw_circle(w / 3.0 * 2.0, h / 3.0 * 2.0, 80.0, PURPLE);

        next_frame().await;
    }
}
