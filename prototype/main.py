import yahoo_fin.stock_info as si
import numpy as np

class Stock:
	def __init__(self, id):
		self.__name = ""
		self.__total_amount = np.float32(0)
		self.__total_investment = np.float32(0)
		self.__stock_transactions = []
		self.__name = id
	
	def __add_purchase(self, amount, price):
		self.__total_amount += amount
		self.__total_investment += amount * price
		self.__stock_transactions.append((amount, price))

	def __add_sell(self, amount, price):
		self.__total_amount += amount
		self.__total_investment += amount * price
		if (self.__total_amount == 0):
			self.__stock_transactions = []
			self.__total_investment *= -1
			self.__total_amount = 0
			return
		new_price = 0
		for t in self.__stock_transactions:
			new_price += t[1]
		new_price /= len(self.__stock_transactions)
		self.__stock_transactions = []
		self.__stock_transactions.append((self.__total_amount, new_price))

	def add_transaction(self, amount, price):
		amount = np.float32(amount)
		price = np.float32(price)
		if ".DE" in self.__name:
			price *= 1.08
		if amount < 0:
			self.__add_sell(amount, price)
		else:
			self.__add_purchase(amount, price)
	
	def get_name(self):
		return self.__name

	def get_transaction_list(self):
		return self.__stock_transactions

	def get_total_investment(self):
		return self.__total_investment
	
	def get_current_value(self, current_price):
		return self.__total_amount * current_price
	
	def get_current_potential(self, current_price):
		if (self.__total_amount > 0):
			potential = (self.__total_amount * current_price) - self.__total_investment
		else:
			potential = 0
		return potential
	
	def show_transactions(self):
		sub_total = 0
		for t in self.__stock_transactions:
			if (t[0] > 0):
				investment = t[0] * t[1]
				sub_total += investment
				print(f"BUY  {t[0]} at {t[1]} -> {investment} {sub_total}")
			else:
				investment = t[0] * t[1]
				sub_total += investment
				print(f"SELL {t[0]} at {t[1]} -> {investment} {sub_total}")

	def display_info(self, current_price):
		print(f"Stock code       : {self.__name}")
		if self.__total_amount <= 0:
			print(f"Overall Status   : {self.__total_investment}")
			return
		print(f"Stock amount     : {self.__total_amount}")
		print(f"Stock value      : {current_price}")
		print(f"Total investment : {self.__total_investment}")
		print("=>")
		print(f"current value     = {self.get_current_value(current_price)}")
		print(f"potential gain    = {self.get_current_potential(current_price)}")

def create_stocks_list(transaction_list):
	stock_ids = [t["id"] for t in transaction_list]
	stock_ids = set(stock_ids)
	stock_list = []
	for id in stock_ids:
		stock_list.append(Stock(id))

	for stock in stock_list:
		for transaction in transaction_list:
			if (stock.get_name() == transaction["id"]):
				stock.add_transaction(transaction["amount"], transaction["price"])
	return stock_list

def parse_transactions(data):
	transaction = {
		"id" : "",
		"amount" : 0.0,
		"date" : "",
		"hour" : "",
		"price" : 0.0
	}
	for index, key in enumerate(transaction.keys()):
		if (key == "amount"):
			transaction[key] = np.float32(data.split()[index])
		elif (key == "price"):
			transaction[key] = np.float32(data.split()[index])
		else:
			transaction[key] = data.split()[index]
	return transaction

def retrieve_transactions(file_path):
	transaction_list = []
	with open(file_path, "r") as f:
		for line in f:
			transaction = parse_transactions(line)
			transaction_list.append(transaction)

	return transaction_list

def main():
	transaction_list = retrieve_transactions("../data/transactions.txt")
	stock_list = create_stocks_list(transaction_list)
	total = 0
	t_potential = 0
	discarded = 0
	for stock in stock_list:
		current_price = si.get_live_price(stock.get_name())
		print("===============================================")
		stock.display_info(current_price)
		stock.show_transactions()
		total += stock.get_current_value(current_price)
		t_potential += stock.get_current_potential(current_price)
		if (stock.get_current_value(current_price) < 0.001):
			discarded += stock.get_total_investment()
	print(f"total money invested  = {total} $")
	print(f"total money discarded = {discarded} $")
	print(f"total potential gain  = {t_potential} $")

main()