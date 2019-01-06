use crate::screen::Screen;
use crate::data::StockDataDisplay;
use crate::output::MyWindow;
use pancurses::Window;

pub struct ListScreen {
    data: Vec<StockDataDisplay>,
    win: MyWindow
}

impl ListScreen {
    pub fn new(win: MyWindow) -> ListScreen {
        ListScreen{
            data: vec![],
            win: win
        }
    }
}

impl Screen<Vec<StockDataDisplay>> for ListScreen {
    fn update_state(&mut self, state: Box<Vec<StockDataDisplay>>){
        self.data = *state;
    }

    fn paint(&mut self){
        self.win.win.mvaddstr(10, 10, "TEST");
    }

    fn clear(&mut self){
        self.win.win.erase();
    }

    fn refresh(&mut self){
        self.win.win.refresh();
    }

}

