use crate::listscreen::ListScreen;
use crate::screen::Screen;
use crate::data::get_stock_data;
use crate::output::MyWindow;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use pancurses::Input;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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

    let mut stocklist = Vec::new();
    
    match File::open(".rmktsrc"){
        Ok(f) => {
            let file = BufReader::new(&f);
            for line in file.lines() {
                match line {
                    Ok(l) => {
                        stocklist.push(String::from(format!("{}", l).trim()));
                    },
                    Err(_) => continue,
                };
            }
        },
        Err(err) => {
            let int_stocklist = vec!["AAPL", "AMZN", "FB", "GOOGL", "NFLX"];
            stocklist = int_stocklist.iter().map(|x| x.to_string()).collect();
        },
    };

    
    // create a thread to asynchronously fetch state updates
    // and refresh the screen when new data is retreived.
    thread::spawn(move || {
        loop {
            let stocks = get_stock_data(&stocklist);

            let mut thread_state = thread_state.lock().unwrap();
            if (*thread_state).is_finished(){
                drop(thread_state);
                return;
            }
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
            Some(Input::Character('q')) => {
                (*state).finish();
                drop(state);
                break;
            },
            _ => ()
        }
        drop(state);
        // sleep for a bit to allow a chance for the state update thread to get control
        let intvl = time::Duration::from_millis(10);
        thread::sleep(intvl);
    }
}
