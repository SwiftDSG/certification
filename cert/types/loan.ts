export type LoanRequest = {
  book_id: string;
};

export type Loan = {
  _id: string;
  book: {
    _id: string;
    name: string;
  };
  user: {
    _id: string;
    name: string;
  };
  borrow_date: number;
  return_date?: number;
  status: "unreturned" | "returned";
};
