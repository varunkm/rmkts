use crate::screen::Screen;
use crate::data::StockDataDisplay;
use crate::output::MyWindow;
use crate::output::{align_left, align_centre, align_right};
use pancurses::Attribute;
use std::time::{Duration, SystemTime};


const TITLE_ROWS: i32 = 1;

pub struct ListScreen {
    data: Vec<StockDataDisplay>,
    pub win: MyWindow,
    cur_row: i32,
    disp_top: i32,
    disp_bot: i32
}

impl ListScreen {
    pub fn new(win: MyWindow) -> ListScreen {
        ListScreen{
            data: vec![],
            disp_bot: win.win.get_max_y(),
            win: win,
            cur_row: 0,
            disp_top: 0
        }
    }

    pub fn paint_row(&self, data: &StockDataDisplay, row: i32, sel: bool){
        let mut col = 0;
        if (sel){
            self.win.win.attrset(Attribute::Reverse);
        }
        self.win.win.mvaddstr(row, col, data.ticker.clone());
        self.win.win.attrset(Attribute::Normal);
        col += 10;
        self.win.win.attrset(Attribute::Bold);
        self.win.win.mvaddstr(row, col, data.last_price.clone());
        self.win.win.attrset(Attribute::Normal);
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

    pub fn scroll_up(&mut self){
        self.cur_row -= 1;
        if (self.cur_row < 0) {
            self.cur_row = 0;
        }

        if (self.cur_row < self.disp_top) {
            self.disp_top = self.cur_row;
            self.disp_bot -= 1;
        }
    }

    pub fn scroll_dn(&mut self){
        self.cur_row += 1;
        if (self.cur_row >= self.data.len() as i32) {
            self.cur_row = (self.data.len() - 1) as i32;
        }

        if (self.cur_row > self.disp_bot) {
            self.disp_bot = self.cur_row;
            self.disp_top += 1;
        }
    }
    
    pub fn paint_headers(&self, row: i32){
        let headers = vec!["SYM","LAST", "CHG", "CHG%", "O", "H", "L", "C"];
        let mut col = 0;
        self.win.win.attrset(Attribute::Bold);
        for header in headers.iter(){
            self.win.win.mvaddstr(row, col, header);
            col += 10;
        }
        self.win.win.attrset(Attribute::Normal);
    }

    pub fn paint_titles(&mut self){
        let now = SystemTime::now();
        self.win.win.mvaddstr(0,0,"STOCKS");
        self.win.win.mvaddstr(0, 10, format!("{:?}", now));
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
        for (i, d) in self.data.iter().enumerate(){
            self.paint_row(d, row, i==self.cur_row as usize);
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

