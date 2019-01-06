use pancurses::Window;

pub struct MyWindow {
    pub win: Window
}

impl MyWindow {
    pub fn new(win: Window) -> MyWindow {
        MyWindow {
            win: win
        }
    }
}

unsafe impl Send for MyWindow {}
