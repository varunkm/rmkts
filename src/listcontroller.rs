use crate::listscreen::ListScreen;
use crate::screen::Screen;
use crate::data::{StockDataDisplay, get_stock_data};
use std::{thread, time};
use std::sync::{Arc, Mutex};

/*
TODO: 
- Fetch real data
- Handle input
- Add ability to kill thread through some shared flag or a channel
*/

pub fn run() {
    // screen is wrapped in a Mutex to allow for safe concurrent
    // mutation by UI thread and state update thread
    let state = Arc::new(Mutex::new(ListScreen::new()));
    let thread_state = state.clone();

    // create a thread to asynchronously fetch state updates
    // and refresh the screen when new data is retreived.
    thread::spawn(move || {
        loop {
            let stocks = get_stock_data(
                &vec![String::from("AAPL"), String::from("FB")]);
            let mut thread_state = thread_state.lock().unwrap();
            (*thread_state).update_state(Box::new(stocks));
            (*thread_state).paint();
            drop(thread_state);
            let intvl = time::Duration::from_millis(1000);
            thread::sleep(intvl);
        }
    });
    
    // main UI routine: handle input and transitions to other screens
    loop {
        let mut state = state.lock().unwrap();
        // do nothing for now
        drop(state);
        let intvl = time::Duration::from_millis(1000);
        thread::sleep(intvl);
    }
}
