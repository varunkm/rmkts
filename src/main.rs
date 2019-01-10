extern crate pancurses;

use pancurses::*;

mod data;
mod listcontroller;
mod screen;
mod listscreen;
mod output;

fn main() {
    let win = output::MyWindow::new(initscr());
    start_color();
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_WHITE, COLOR_BLUE);
    noecho();
    half_delay(1);
    listcontroller::run(win);
}
