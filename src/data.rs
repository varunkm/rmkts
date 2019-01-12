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

fn extract_stock_data(r: &Value) -> Result<StockData, &'static str> {
    let s = StockData::new(r["symbol"].as_str().unwrap_or(""),
                           r["regularMarketPrice"].as_f64().unwrap_or(0.0),
                           r["regularMarketOpen"].as_f64().unwrap_or(0.0),
                           r["regularMarketDayHigh"].as_f64().unwrap_or(0.0),
                           r["regularMarketDayLow"].as_f64().unwrap_or(0.0),
                           r["regularMarketPreviousClose"].as_f64().unwrap_or(0.0),
                           r["regularMarketChange"].as_f64().unwrap_or(0.0),
                           r["regularMarketVolume"].as_i64().unwrap_or(0),
                           r["marketCap"].as_i64().unwrap_or(0));
    Ok(s)
}

// get_raw_data:
//  - takes a list of tickers, constructs the API url
//  - makes a GET request to this url *
//  - extracts the text from the response *
//  - deserialises the json in the text *
//  - parses the json to produce a list of StockData structs *(on each element)
// * - can produce an error
fn get_raw_data(tickers: &Vec<String>) -> Result<Vec<Result<StockData, &'static str>>, String>{
    // construct request string
    let mut requrl = String::from(BASE_URL);
    for s in tickers{
        requrl.push_str(&s);
        requrl.push(',');
    }
    let mut res = reqwest::get(&requrl).map_err(|e| e.to_string())?;
    let body = res.text().map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let mut ret = Vec::new();
    
    for r in v["quoteResponse"]["result"].as_array().unwrap_or(&vec![]) {
        ret.push(extract_stock_data(r))
    }
    Ok(ret)
}

fn convert_to_display(item: Result<StockData, &'static str>) -> StockDataDisplay {
    match item {
        Ok(data) => data.to_display(),
        Err(data) => {
            StockDataDisplay {
                ticker: String::from("ERR"),
                last_price: String::new(),
                open: String::new(),
                high: String::new(),
                low: String::new(),
                close: String::new(),
                change: String::new(),
                change_p: String::new(),
                volume: String::new(),
                mktcap: String::new(),
            }
        },
    }
}

pub fn get_stock_data(tickers: &Vec<String>) -> Vec<StockDataDisplay>{
    match get_raw_data(tickers) {
        Ok(data) => {
            data.into_iter().map(|x| convert_to_display(x)).collect()
        },
        Err(e) => vec![],
    }
}


