extern crate pancurses;

use std::io;
use pancurses::{initscr, Window, half_delay, noecho};

mod data;
mod listcontroller;
mod screen;
mod listscreen;
mod output;



use std::{thread, time};

fn main() {
    let win = output::MyWindow::new(initscr());
    noecho();
    half_delay(1);
    listcontroller::run(win);
}
