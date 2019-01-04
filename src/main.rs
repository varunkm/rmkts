mod data;
mod listcontroller;
mod screen;
mod listscreen;


fn main() {
//    let stocks = data::get_stock_data(&vec![String::from("AAPL"), String::from("FB")]);
    listcontroller::run();
}
