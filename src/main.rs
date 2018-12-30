mod data;

fn main() {
    let stocks = data::get_stock_data(&vec![String::from("AAPL"), String::from("FB")]);

    for s in stocks {
        println!("{}\t{}", s.ticker, s.last_price)
    }
    
    println!("Hello, world!");
}
