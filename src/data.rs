extern crate serde_json;

use serde_json::Value;

const BASE_URL: &str = "https://query1.finance.yahoo.com/v7/finance/quote?symbols=";

struct StockData {
    ticker: String,
    last_price: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    change: f64,
    change_p: f64,
    volume: i64,
    mktcap: i64
}

pub struct StockDataDisplay {
    pub ticker: String,
    pub last_price: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub change: String,
    pub change_p: String,
    pub volume: String,
    pub mktcap: String
}

// round to two decimal places
fn round(data: f64) -> f64{
    (data * 100.0).round() / 100.0
}    

fn round_format(data: f64) -> String {
    format!("{:.2}", round(data))
}

fn money_format(data: f64) -> String {
    format!("${:.2}", round(data))
}

fn percent_format(data: f64) -> String {
    format!("{:.2}%", round(data * 100.0))
}

impl StockData {
    fn new(ticker: &str, last_price: f64, open: f64, high: f64, low:
           f64, close: f64, change: f64, volume: i64, mktcap: i64) -> StockData
    {
        StockData{
            ticker:String::from(ticker),
            last_price:last_price,
            open:open,
            high:high,
            low:low,
            close:close,
            change:change,
            change_p: change / open,
            volume:volume,
            mktcap:mktcap
        }
    }

    fn to_display(&self) -> StockDataDisplay {
         StockDataDisplay {
             ticker: self.ticker.clone(),
             last_price: money_format(self.last_price),
             open: money_format(self.open),
             high: money_format(self.high),
             low: money_format(self.low),
             close: money_format(self.close),
             change: round_format(self.change),
             change_p: percent_format(self.change_p),
             volume: self.volume.to_string(),
             mktcap: self.mktcap.to_string()
         }
    }
}

fn get_raw_data(tickers: &Vec<String>) -> Vec<StockData>{
    // construct request string
    let mut requrl = String::from(BASE_URL);
    for s in tickers{
        requrl.push_str(&s);
        requrl.push(',');
    }
    let mut res = reqwest::get(&requrl).unwrap();
    let body = res.text().unwrap();
    let v: Value = serde_json::from_str(&body).unwrap();
    let mut ret = Vec::new();
    
    for r in v["quoteResponse"]["result"].as_array().unwrap() {
        ret.push(StockData::new(r["symbol"].as_str().unwrap(),
                                r["regularMarketPrice"].as_f64().unwrap(),
                                r["regularMarketOpen"].as_f64().unwrap(),
                                r["regularMarketDayHigh"].as_f64().unwrap(),
                                r["regularMarketDayLow"].as_f64().unwrap(),
                                r["regularMarketPreviousClose"].as_f64().unwrap(),
                                r["regularMarketChange"].as_f64().unwrap(),
                                r["regularMarketVolume"].as_i64().unwrap(),
                                r["marketCap"].as_i64().unwrap()));
    }
    ret
}

pub fn get_stock_data(tickers: &Vec<String>) -> Vec<StockDataDisplay>{
    get_raw_data(tickers).into_iter().map(|x| x.to_display()).collect()
}


