use yahoo_finance_api as yahoo;
use tokio_test;

pub mod file_ops;
pub mod stock_logic;
use crate::file_ops::*;

fn stock_get_current_price(symbol: String, provider: &yahoo::YahooConnector) -> f32
{
	let response = tokio_test::block_on(provider.get_latest_quotes(&symbol, "1d")).unwrap();
	response.last_quote().unwrap().close as f32
}

fn main()
{
	let data_path = create_relative_path("data/transactions.txt");
	let mut stock_vec = create_stock_vec(data_path);
	let provider = yahoo::YahooConnector::new();
	let mut total_potential = 0.0;
	for stock in stock_vec.iter_mut() {
		println!("====================");
		let cur_price = stock_get_current_price(stock.get_symbol(), &provider);
		stock.update_current_price(cur_price);
		stock.print_stock();
		total_potential += stock.get_potential();
	}
	println!("current  potential = {}", total_potential);
}