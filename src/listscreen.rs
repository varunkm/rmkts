
use crate::screen::Screen;
use crate::data::StockDataDisplay;
use crate::output::{MyWindow, align_right, align_centre};
use pancurses::*;
use std::time::SystemTime;


const TABLE_OFFSET: i32 = 2;

pub struct ListScreen {
    data: Vec<StockDataDisplay>,
    pub win: MyWindow,
    cur_row: i32,
    table_start: i32,
    table_end: i32
}

impl ListScreen {
    pub fn new(win: MyWindow) -> ListScreen {
        ListScreen{
            data: vec![],
            table_start: 0,
            table_end: win.win.get_max_y() - TABLE_OFFSET - 1,
            cur_row: 0,
            win: win,
        }
    }

    pub fn paint_row(&self, data: &StockDataDisplay, row: i32, sel: bool){
        let mut col = 0;
        let neg: bool = data.change.chars().next().unwrap() == '-';
        let change_col = {
            if neg{
                1
            }
            else {
                2
            }
        };

        
        if sel {
            self.win.win.attrset(Attribute::Reverse);
        }
        self.win.win.mvaddstr(row, col, data.ticker.clone());
        
        self.win.win.attrset(Attribute::Normal);
        col += 10;
        
        self.win.win.attrset(Attribute::Bold);
        self.win.win.mvaddstr(row, col, align_right(data.last_price.clone(), 10));
        self.win.win.attrset(Attribute::Normal);
        col += 10;

        self.win.win.attrset(ColorPair(change_col));
        self.win.win.mvaddstr(row, col, align_right(data.change.clone(), 10));
        col += 10;
        
        self.win.win.mvaddstr(row, col, align_right(data.change_p.clone(), 10));
        col += 10;
        self.win.win.attrset(Attribute::Normal);
        self.win.win.mvaddstr(row, col, align_right(data.open.clone(), 10));
        col += 10;
        
        self.win.win.mvaddstr(row, col, align_right(data.high.clone(), 10));
        col += 10;
        
        self.win.win.mvaddstr(row, col, align_right(data.low.clone(), 10));
        col += 10;
        
        self.win.win.mvaddstr(row, col, align_right(data.close.clone(), 10));
    }

    pub fn resize(&mut self){
        let table_cap = self.win.win.get_max_y() - TABLE_OFFSET - 1;
        self.table_end = self.table_start + table_cap;
        if self.cur_row > self.table_end {
            self.cur_row = self.table_end - 1;
        }
    }
    
    pub fn scroll_up(&mut self){
        self.cur_row -= 1;
        if self.cur_row < 0 {
            self.cur_row = 0;
        }

        if self.cur_row < self.table_start {
            self.table_start = self.cur_row;
            self.table_end -= 1;
        }
    }

    pub fn scroll_dn(&mut self){
        self.cur_row += 1;
        if self.cur_row > self.data.len() as i32 {
            self.cur_row = (self.data.len() - 1) as i32;
        }

        if self.cur_row > self.table_end {
            self.table_end = self.cur_row;
            self.table_start += 1;
        }
    }
    
    pub fn paint_headers(&self, row: i32){
        let headers = vec!["SYM","LAST", "CHG", "CHG%", "O", "H", "L", "C"];
        let mut col = 0;
        self.win.win.attrset(Attribute::Bold);
        for header in headers.iter(){
            self.win.win.mvaddstr(row, col, align_centre(header.to_string(), 10));
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
        self.resize();
        self.paint_titles();
        let mut row = 1;
        self.paint_headers(row);
        row += 1;
        for (i, d) in self.data.iter().enumerate() {
            if i < self.table_start as usize || i > self.table_end as usize {
                continue;
            }
            
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

