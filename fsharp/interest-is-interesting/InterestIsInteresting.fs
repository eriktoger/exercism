module InterestIsInteresting

let interestRate (balance: decimal) : single =
    if balance < 0.0m then 3.213f
    elif balance < 1000m then 0.5f
    elif balance < 5000m then 1.621f
    else 2.475f

let interest (balance: decimal) : decimal =
    decimal (interestRate balance) * balance / 100m

let annualBalanceUpdate (balance: decimal) : decimal = balance + (interest balance)

let amountToDonate (balance: decimal) (taxFreePercentage: float) : int =
    if balance < 0.0m then
        0
    else
        int ((taxFreePercentage * 2.0 / 100.0) * float (balance))
