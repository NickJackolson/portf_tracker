#[derive(Debug, Clone, Copy)]
pub struct Transaction {
	amount: f32,
	price: f32
}

impl Transaction
{
	pub fn new(amount: f32, price: f32) -> Transaction {
		Transaction {
			amount: amount,
			price: price,
		}
	}
}

pub struct Stock {
	symbol: String,
	total_amount: f32,
	total_investment: f32,
	transactions: Vec<Transaction>,
	current_price: f32
}

impl Stock {
	pub fn new(symbol: String) -> Stock {
		Stock {
			symbol: symbol,
			total_amount: 0.0,
			total_investment: 0.0,
			transactions: Vec::new(),
			current_price: 0.0,
		}
	}
	
	pub fn get_symbol(&self) -> String
	{
		self.symbol.clone()
	}

	pub fn update_current_price(&mut self, current_price: f32)
	{
		self.current_price = current_price;
		if self.symbol.contains("DE") {
			self.current_price *= 1.1;
		}
	}

	fn add_transact_sale(&mut self)
	{
		let avg_price: f32 = self.transactions
		.iter()
		.fold(0.0, |acc: f32, x| acc + x.price) / self.total_amount as f32;
		let after_sale: Transaction = Transaction {
			amount : self.total_amount,
			price : avg_price,
		};
		self.transactions.clear();
		self.transactions.push(after_sale);
	}

	fn add_transact_buy(&mut self, t: Transaction)
	{
		self.transactions.push(t);
	}

	fn transact_empty(&mut self)
	{
			self.total_amount = 0.0;
			self.total_investment *= -1.0;
			self.transactions.clear();
	}

	pub fn add_transaction(&mut self, t: Transaction)
	{
		self.total_amount += t.amount;
		self.total_investment += t.price * t.amount;
		if t.amount < 0.0 {
			self.add_transact_sale();
		} else {
			self.add_transact_buy(t);
		}
		if self.total_amount <= 0.00001 {
			self.transact_empty();
		}
	}

	pub fn print_stock(&self)
	{
		println!("Showing {}:", self.symbol);
		println!("Amount         : {}", self.total_amount);
		if self.total_amount > 0.0 {
			println!("Invested Money : {}", self.total_investment);
		} else {
			println!("Gained Money   : {}", self.total_investment);
		}
		let current_value = self.total_amount * self.current_price;
		let potential = self.get_potential();
		println!("Current Value  : {}", current_value);
		println!("Potential Gain : {}", potential);
	}
	
	pub fn get_potential(&self) -> f32
	{
		if self.total_amount == 0.0 {
			return 0.0;
		}
		return self.total_amount * self.current_price - self.total_investment;
	}
}
