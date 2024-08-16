use raylib::prelude::*;

static WINDOW_WIDTH: i32 = 680;
static WINDOW_HEIGHT: i32 = 440;

fn main() {
    let (mut lib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("My Pomodoro")
        .build();

    while !lib.window_should_close() {
        let mut draw = lib.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw_button(draw);
    }
}

fn draw_button(mut draw: RaylibDrawHandle) {
    let button_width = WINDOW_WIDTH / 5;
    let button_height = WINDOW_HEIGHT / 8;

    let border = Rectangle::new(
        (WINDOW_WIDTH / 2 - button_width / 2) as f32,
        (WINDOW_HEIGHT / 2) as f32,
        (button_width) as f32,
        (button_height) as f32,
    );
    draw.draw_rectangle_rounded(border, 0.2, 1, Color::WHITE);

    let rec = Rectangle::new(
        (WINDOW_WIDTH / 2 - button_width / 2) as f32,
        (WINDOW_HEIGHT / 2) as f32,
        (button_width) as f32,
        (button_height) as f32,
    );

    draw.draw_rectangle_rounded(rec, 0.2, 1, Color::REBECCAPURPLE);
}
