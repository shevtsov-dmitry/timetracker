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
    }
}
