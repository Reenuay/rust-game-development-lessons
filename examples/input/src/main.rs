use macroquad::prelude::*;

// Liberation Sans Bold (SIL Open Font License 1.1, https://github.com/liberationfonts) —
// macroquad's own built-in font only covers ASCII, so a Cyrillic-capable
// font is needed to draw these labels.
const FONT_BYTES: &[u8] = include_bytes!("../assets/LiberationSans-Bold.ttf");

fn draw_centered(text: &str, font: &Font, font_size: u16, x: f32, y: f32, color: Color) {
    let size = measure_text(text, Some(font), font_size, 1.0);
    let params = TextParams {
        font: Some(font),
        font_size,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x - size.width / 2.0, y + size.height / 2.0, params);
}

#[macroquad::main("Ввод с клавиатуры")]
async fn main() {
    let font = load_ttf_font_from_bytes(FONT_BYTES).unwrap();
    let font_size = 100;

    loop {
        clear_background(BLACK);

        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let offset_x = screen_width() * 0.16;
        let offset_y = screen_height() * 0.145;

        if is_key_down(KeyCode::Up) {
            draw_centered("Вверх", &font, font_size, cx, cy - offset_y, YELLOW);
        }
        if is_key_down(KeyCode::Down) {
            draw_centered("Вниз", &font, font_size, cx, cy + offset_y, YELLOW);
        }
        if is_key_down(KeyCode::Left) {
            draw_centered("Влево", &font, font_size, cx - offset_x, cy, YELLOW);
        }
        if is_key_down(KeyCode::Right) {
            draw_centered("Вправо", &font, font_size, cx + offset_x, cy, YELLOW);
        }

        next_frame().await;
    }
}
