extern crate pancurses;

use std::io;
use pancurses::{initscr, Window};

mod data;
mod listcontroller;
mod screen;
mod listscreen;
mod output;



use std::{thread, time};

fn main() {
    let win = output::MyWindow::new(initscr());
    listcontroller::run(win);

}
