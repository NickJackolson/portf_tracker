use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, BufReader, Lines};
use std::path::Path;
use std::path::PathBuf;
use crate::stock_logic::{Transaction, Stock};

pub fn create_relative_path(path: &str) -> PathBuf
{
    let cwd_absolute = Path::new(".").canonicalize().unwrap();
	let data_path = cwd_absolute.join(path);
    return data_path;
}

fn stock_vec_add_transaction(mut stock_vec: Vec<Stock>, symbol: String, t: Transaction) -> Vec<Stock>
{
	for stock in stock_vec.iter_mut() {
		if stock.get_symbol() == symbol {
			stock.add_transaction(t);
			return stock_vec;
		}
	}
	let mut new_stock = Stock::new(symbol);
	new_stock.add_transaction(t);
	stock_vec.push(new_stock);
	return stock_vec;
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>
{
	let file = File::open(filename)?;
	Ok(BufReader::new(file).lines())
}

pub fn create_stock_vec(path: PathBuf) -> Vec<Stock>
{
	let mut stock_vec: Vec<Stock> = Vec::new();

	if let Ok(lines) = read_lines(path) {
		for line in lines {
			if let Ok(transaction_line) = line {
				let v : Vec<&str> = transaction_line.split(' ').collect();
				let symbol = String::from(v[0]);
				let amount = v[1].parse::<f32>().unwrap();
				let price = v[4].parse::<f32>().unwrap();
				let t = Transaction::new(amount, price);
				stock_vec = stock_vec_add_transaction(stock_vec, symbol, t);
			}
		}
	}
	return stock_vec;
}