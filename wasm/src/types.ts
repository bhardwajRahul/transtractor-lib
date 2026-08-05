export interface Transaction {
  date: number;
  index: number;
  description: string;
  amount: number;
  balance: number;
}

export interface StatementData {
  key: string;
  account_number: string;
  start_date: number;
  opening_balance: number;
  closing_balance: number;
  transactions: Transaction[];
}
