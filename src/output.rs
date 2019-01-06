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

pub fn align_left(s: String, space: usize) -> String {
    let trunc: String = s.chars().take(space).collect();
    let diff: usize = space - trunc.chars().count();
    let spaces = std::iter::repeat(" ").take(diff).collect::<String>();
    format!("{}{}", spaces,trunc)
}

pub fn align_centre(s: String, space: usize) -> String {
    let trunc: String = s.chars().take(space).collect();
    let diff = space - trunc.chars().count();
    let spaces_left = (std::iter::repeat(" ").take(diff / 2).collect::<String>());
    let spaces_right = (std::iter::repeat(" ").take(diff / 2 + diff % 2).collect::<String>()); 
    format!("{}{}{}", spaces_left, trunc, spaces_right)
}

pub fn align_right(s: String, space: usize) -> String {
    let trunc: String = s.chars().take(space).collect();
    let diff = space - trunc.chars().count();
    let spaces = std::iter::repeat(" ").take(diff).collect::<String>();
    format!("{}{}", trunc, spaces)
}
