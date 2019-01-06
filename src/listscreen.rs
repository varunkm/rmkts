use crate::screen::Screen;
use crate::data::StockDataDisplay;
use crate::output::MyWindow;
use crate::output::{align_left, align_centre, align_right};


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

    pub fn paint_row(&self, data: &StockDataDisplay, row: i32){
        let mut col = 0;
        self.win.win.mvaddstr(row, col, data.ticker.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.last_price.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.change.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.change_p.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.open.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.high.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.low.clone());
        col += 10;
        self.win.win.mvaddstr(row, col, data.close.clone());
    }

    pub fn paint_headers(&self, row: i32){
        let headers = vec!["SYM","LAST", "CHG", "CHG%", "O", "H", "L", "C"];
        let mut col = 0;
        for header in headers.iter(){
            self.win.win.mvaddstr(row, col, header);
            col += 10;
        }
    }

    pub fn paint_titles(&mut self){
        self.win.win.mvaddstr(0,0,"STOCKS");
    }
        
}

impl Screen<Vec<StockDataDisplay>> for ListScreen {
    fn update_state(&mut self, state: Box<Vec<StockDataDisplay>>){
        self.data = *state;
    }

    fn paint(&mut self){
        self.paint_titles();
        let mut row = 1;
        self.paint_headers(row);
        row += 1;
        for d in self.data.iter(){
            self.paint_row(d, row);
            row += 1;
        }
    }

    fn clear(&mut self){
        self.win.win.erase();
    }

    fn refresh(&mut self){
        self.win.win.refresh();
    }

}

