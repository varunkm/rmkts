use crate::listscreen::ListScreen;
use crate::screen::Screen;
use crate::data::{StockDataDisplay, get_stock_data};
use crate::output::MyWindow;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use pancurses::{Window, Input};
/*
TODO: 
- Fetch real data
- Handle input
- Add ability to kill thread through some shared flag or a channel
*/

pub fn run(win: MyWindow) {
    // screen is wrapped in a Mutex to allow for safe concurrent
    // mutation by UI thread and state update thread

    let state = Arc::new(Mutex::new(ListScreen::new(win)));
    let thread_state = state.clone();

    // create a thread to asynchronously fetch state updates
    // and refresh the screen when new data is retreived.
    thread::spawn(move || {
        loop {
            let stocks = get_stock_data(
                &vec![String::from("AAPL"), String::from("FB")]);

            let mut thread_state = thread_state.lock().unwrap();
            (*thread_state).update_state(Box::new(stocks));
            (*thread_state).clear();
            (*thread_state).paint();
            (*thread_state).refresh();
            drop(thread_state);
            let intvl = time::Duration::from_millis(1000);
            thread::sleep(intvl);
        }
    });
    
    // main UI routine: handle input and transitions to other screens
    let mut x = 1;
    loop {
        let mut state = state.lock().unwrap();
        match (*state).win.win.getch() {
            Some(Input::Character('j')) => {
                (*state).clear();
                (*state).scroll_dn();
                (*state).paint();
                (*state).refresh();
            },
            Some(Input::Character('k')) => {
                (*state).clear();
                (*state).scroll_up();
                (*state).paint();
                (*state).refresh();
            },
            _ => ()
        }
        drop(state);
        let intvl = time::Duration::from_millis(100);
        thread::sleep(intvl);
    }
}
