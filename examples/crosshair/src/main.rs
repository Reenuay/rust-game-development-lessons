use macroquad::prelude::*;

#[macroquad::main("Свой курсор")]
async fn main() {
    // Прячем обычную стрелку мыши. Делаем это один раз, до цикла.
    show_mouse(false);

    loop {
        clear_background(BLACK);

        // Координаты мыши прямо сейчас.
        let (x, y) = mouse_position();

        // Свой курсор — прицел. Кольцо вокруг курсора.
        draw_circle_lines(x, y, 20.0, 3.0, RED);
        // Горизонтальная черта через центр.
        draw_line(x - 30.0, y, x + 30.0, y, 3.0, RED);
        // Вертикальная черта через центр.
        draw_line(x, y - 30.0, x, y + 30.0, 3.0, RED);

        next_frame().await;
    }
}
