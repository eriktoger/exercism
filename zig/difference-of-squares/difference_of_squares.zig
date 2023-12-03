pub fn squareOfSum(number: usize) usize {
    var num: usize = number;

    var result: usize = 0;
    while (num > 0) {
        result += num;
        num -= 1;
    }
    return result * result;
}

pub fn sumOfSquares(number: usize) usize {
    var num: usize = number;

    var result: usize = 0;
    while (num > 0) {
        result += num * num;
        num -= 1;
    }
    return result;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
