use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, BufReader, Lines};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
struct Transaction {
	amount: f32,
	price: f32
}

struct Stock {
	symbol: String,
	total_amount: f32,
	total_investment: f32,
	transactions: Vec<Transaction>
}

impl Stock {
	fn new(symbol: String) -> Stock {
		Stock {
			symbol: symbol,
			total_amount: 0.0,
			total_investment: 0.0,
			transactions: Vec::new(),
		}
	}

	fn add_transaction(&mut self, t: Transaction)
	{
		self.total_amount += t.amount;
		if self.total_amount <= 0.00001 {
			self.total_amount = 0.0;
			self.total_investment += t.price * t.amount;
			self.total_investment *= -1.0;
			return;
		}
		if t.amount < 0.0 && self.total_amount > 0.0 {
			let avg_price: f32 = self.transactions
			.iter()
			.fold(0.0, |acc: f32, x| acc + x.price) / self.transactions.len() as f32;
			let after_sale: Transaction = Transaction {
				amount : self.total_amount,
				price : avg_price,
			};
			self.transactions = vec![after_sale];
		}  else {
			self.transactions.push(t);
		}
		self.total_investment += t.price * t.amount;
	}

	fn print_stock(&self)
	{
		println!("Showing {}:", self.symbol);
		println!("Amount         : {}", self.total_amount);
		if self.total_amount > 0.0 {
			println!("Invested Money : {}", self.total_investment);
		} else {
			println!("Gained Money   : {}", self.total_investment);
		}
	}

}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>
{
	let file = File::open(filename)?;
	Ok(BufReader::new(file).lines())
}

fn stock_vec_add_transaction(mut stock_vec: Vec<Stock>, symbol: String, t: Transaction) -> Vec<Stock>
{
	for stock in stock_vec.iter_mut() {
		if stock.symbol == symbol {
			stock.add_transaction(t);
			return stock_vec;
		}
	}
	let mut new_stock = Stock::new(symbol);
	new_stock.add_transaction(t);
	stock_vec.push(new_stock);
	return stock_vec;
}

fn create_stock_list(path: PathBuf) -> Vec<Stock>
{
	let mut stock_vec: Vec<Stock> = Vec::new();

	if let Ok(lines) = read_lines(path) {
		for line in lines {
			if let Ok(transaction_line) = line {
				let v : Vec<&str> = transaction_line.split(' ').collect();
				let symbol = String::from(v[0]);
				let t = Transaction {amount: v[1].parse::<f32>().unwrap(),
												  price: v[4].parse::<f32>().unwrap()};
				stock_vec = stock_vec_add_transaction(stock_vec, symbol, t);
			}
		}
	}
	return stock_vec;
}

fn main()
{
	let cwd_absolute = Path::new(".").canonicalize().unwrap();
	let data_path = cwd_absolute.join("data").join("transactions.txt");
	let stock_vec = create_stock_list(data_path);
	for stock in stock_vec.iter() {
		println!("====================");
		stock.print_stock();
	}
}