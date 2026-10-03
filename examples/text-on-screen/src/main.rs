use macroquad::prelude::*;

#[macroquad::main("Текст на экране")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Текст рисуется от базовой линии — она идёт понизу букв,
        // а не через центр и не по верхнему краю.
        draw_text("Hello!", mouse_x, mouse_y, 40.0, WHITE);

        // Точка ровно в (mouse_x, mouse_y) — видно, где проходит
        // базовая линия относительно самих букв.
        draw_circle(mouse_x, mouse_y, 3.0, RED);

        next_frame().await;
    }
}
