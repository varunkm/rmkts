use crate::screen::Screen;
use crate::data::StockDataDisplay;

pub struct ListScreen {
    data: Vec<StockDataDisplay>
}

impl ListScreen {
    pub fn new() -> ListScreen {
        ListScreen{
            data: vec![]
        }
    }
}

impl Screen<Vec<StockDataDisplay>> for ListScreen {
    fn update_state(&mut self, state: Box<Vec<StockDataDisplay>>){
        self.data = *state;
    }

    fn paint(&mut self){
        for r in self.data.iter(){
            println!("{}\t{}", r.ticker, r.last_price);
        }
    }

    fn clear(&mut self){
    }

    fn refresh(&mut self){
    }

}

